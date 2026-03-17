use crate::dictionary::DictionaryMessage::Test;
use crate::dictionary_test::DictionaryQuizState;
use crate::{NavigatedPage, Page, RootMessage};
use iced::alignment::Vertical::Center;
use iced::widget::button::Style;
use iced::widget::*;
use iced::{Border, Color, Length, Shadow, Task};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;
use DictionaryMessage::Back;

#[derive(Clone, Debug)]
pub struct DictionaryState {
    dict: Vec<DictionaryElement>,
    include_map: Vec<bool>,
    tag_map: HashMap<String, bool>,
    reverse: bool,
    search: String,
    no_typing: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DictionaryElement {
    pub key: String,
    pub value: String,
    pub tags: String,
}

impl DictionaryElement {
    fn new() -> Self {
        Self {
            key: String::new(),
            value: String::new(),
            tags: String::new(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum DictionaryMessage {
    Back,
    SetTags(usize, String),
    SetKey(usize, String),
    SetValue(usize, String),
    Remove(usize),
    NewWord,
    Include(usize, bool),
    IncludeTag(String, bool),
    Save,
    Test,
    ResetTags,
    SetReverse(bool),
    Search(String),
    SetTyping(bool),
}

impl NavigatedPage<DictionaryMessage> for DictionaryState {
    fn navigate(&self, message: &DictionaryMessage) -> Option<Page> {
        if let Back = message {
            return Some(Page::PreviousPage);
        }
        if let Test = message {
            if self.include_map.iter().any(|x| *x) {
                let mut words = vec![];
                for i in 0..self.include_map.len() {
                    if self.include_map[i] {
                        words.push(self.dict[i].clone());
                    }
                }
                return Some(Page::DictionaryQuiz(DictionaryQuizState::new(
                    words,
                    self.reverse,
                    self.no_typing,
                )));
            }
        }
        None
    }
}

impl Default for DictionaryState {
    fn default() -> Self {
        Self::new()
    }
}
impl DictionaryState {
    pub fn new() -> Self {
        let mut current_dict = "[]".to_string();
        match File::open(dict_file()) {
            Ok(mut f) => {
                current_dict = String::new();
                f.read_to_string(&mut current_dict).unwrap();
            }
            _ => {}
        }
        let list: Vec<DictionaryElement> = serde_json::from_str(&current_dict).unwrap();
        let mut result = DictionaryState {
            include_map: vec![false; list.len()],
            dict: list,
            tag_map: HashMap::new(),
            reverse: false,
            search: "".to_string(),
            no_typing: false,
        };

        result.update_tags();

        result
    }

    pub fn update(&mut self, message: DictionaryMessage) -> Task<RootMessage> {
        match message {
            DictionaryMessage::NewWord => {
                self.dict.push(DictionaryElement::new());
                self.include_map.push(false);
            }
            DictionaryMessage::SetKey(i, v) => self.dict[i].key = v,
            DictionaryMessage::SetValue(i, v) => self.dict[i].value = v,
            DictionaryMessage::SetTags(i, v) => {
                self.dict[i].tags = v;
                self.update_tags();
            }
            DictionaryMessage::Remove(i) => {
                self.dict.remove(i);
            }
            DictionaryMessage::Include(i, b) => self.include_map[i] = b,
            DictionaryMessage::IncludeTag(t, v) => {
                self.tag_map.insert(t, v);
                self.update_words_include()
            }
            DictionaryMessage::ResetTags => {
                self.tag_map.iter_mut().for_each(|(_, v)| *v = false);
                self.include_map.iter_mut().for_each(|x| *x = false)
            }
            DictionaryMessage::Save => {
                let dir = dict_file();
                let content = serde_json::to_string_pretty(&self.dict.clone()).unwrap();
                fs::write(dir, content.clone())
                    .unwrap_or_else(|e| println!("Can't write file: {}", e));
            }
            DictionaryMessage::SetReverse(v) => self.reverse = v,
            DictionaryMessage::Search(s) => {
                self.search = s;
            }
            DictionaryMessage::SetTyping(b) => self.no_typing = b,
            _ => {}
        }

        Task::none()
    }

    pub fn view(&self) -> iced::Element<'_, DictionaryMessage> {
        container(
            row![
                iced::widget::column![
                    button("Назад").on_press(Back),
                    self.words_list(),
                    row![
                        button("Добавить слово").on_press(DictionaryMessage::NewWord),
                        button("Сохранить словарь").on_press(DictionaryMessage::Save),
                    ]
                    .spacing(10)
                ]
                .spacing(5),
                self.filters()
            ]
            .spacing(10),
        )
        .padding(10)
        .into()
    }

    fn words_list(&self) -> iced::Element<'_, DictionaryMessage> {
        let mut col = Column::new().width(Length::Fill);

        let mut i = 0;
        for word in &self.dict {
            if !self.search.is_empty() {
                if word.key.contains(&self.search) == false
                    && word.value.contains(&self.search) == false
                {
                    i += 1;
                    continue;
                }
            }

            let mut line = Row::new().width(Length::Fill).align_y(Center);
            line = line
                .push(
                    checkbox(self.include_map[i])
                        .on_toggle(move |b| DictionaryMessage::Include(i, b)),
                )
                .push(space().width(10));

            line = line.push(
                text_input("Ключ", &word.key)
                    .size(20)
                    .width(Length::Fill)
                    .on_input(move |string| DictionaryMessage::SetKey(i, string)),
            );
            line = line.push(
                text_input("Значение", &word.value)
                    .size(20)
                    .width(Length::Fill)
                    .on_input(move |string| DictionaryMessage::SetValue(i, string)),
            );
            line = line.push(
                text_input("Тэги", &word.tags)
                    .size(20)
                    .width(Length::Fill)
                    .on_input(move |string| DictionaryMessage::SetTags(i, string)),
            );

            line = line
                .push(
                    button("-")
                        .on_press_with(move || DictionaryMessage::Remove(i))
                        .style(|_x, _status| Style {
                            background: None,
                            text_color: Color::BLACK,
                            border: Border::default(),
                            shadow: Shadow::default(),
                            snap: false,
                        }),
                )
                .push(space().width(10));

            col = col.push(line);
            i += 1;
        }

        scrollable(col).height(Length::Fill).into()
    }

    fn filters(&self) -> iced::Element<'_, DictionaryMessage> {
        iced::widget::column![
            text_input("Поиск", &self.search)
                .on_input(DictionaryMessage::Search)
                .width(Length::Fill),
            text!("Всего слов: {}", self.dict.len()),
            text!(
                "Выбрано слов: {}",
                self.include_map.iter().filter(|i| **i).count()
            ),
            self.tags_selector(),
            toggler(self.no_typing)
                .label("Без набора")
                .on_toggle(DictionaryMessage::SetTyping),
            toggler(self.reverse)
                .label("Обратный тест")
                .on_toggle(DictionaryMessage::SetReverse),
            button(text!("Тест").center().width(Length::Fill))
                .on_press(Test)
                .width(Length::Fill),
        ]
        .width(250)
        .spacing(10)
        .into()
    }

    fn tags_selector(&self) -> iced::Element<'_, DictionaryMessage> {
        let mut col = Column::new().width(Length::Fill);
        col = col.push(
            button("Сбросить")
                .on_press(DictionaryMessage::ResetTags)
                .style(|x: &Theme, _status| Style {
                    background: None,
                    text_color: x.palette().primary,
                    border: Border::default(),
                    shadow: Shadow::default(),
                    snap: false,
                }),
        );
        let mut sorted_tags = self.tag_map.iter().collect::<Vec<_>>();
        sorted_tags.sort();
        for tag in sorted_tags {
            col = col.push(
                checkbox(*tag.1)
                    .label(tag.0)
                    .on_toggle(|x1| DictionaryMessage::IncludeTag(tag.0.clone(), x1)),
            )
        }

        container(scrollable(col)).height(Length::Fill).into()
    }

    fn update_tags(&mut self) {
        let mut tags_list: Vec<String> = vec![];
        for element in &self.dict {
            tags_list.append(&mut split_with_coma(element.tags.clone()));
        }

        let current_tags = self
            .tag_map
            .keys()
            .map(|k| k.clone().to_string())
            .collect::<Vec<String>>();
        for current in current_tags {
            if !tags_list.contains(&current) {
                self.tag_map.remove(&current.clone());
            }
        }

        for found_tag in &tags_list {
            if !self.tag_map.contains_key(&found_tag.clone()) {
                self.tag_map.insert(found_tag.clone(), false);
            }
        }
    }

    fn update_words_include(&mut self) {
        let include_tags = self
            .tag_map
            .iter()
            .filter(|i| *(*i).1)
            .map(|(t, _)| t.clone())
            .collect::<Vec<String>>();

        if include_tags.is_empty() {
            self.include_map.iter_mut().for_each(|x| *x = false);
            return;
        }

        for i in 0..self.include_map.len() {
            let tags = split_with_coma(self.dict[i].tags.clone());
            if tags.iter().all(|t| include_tags.contains(t)) {
                self.include_map[i] = true;
            } else {
                self.include_map[i] = false;
            }
        }
    }
}

pub fn split_with_coma(ts: String) -> Vec<String> {
    ts.split(',')
        .map(|ts| ts.to_lowercase().trim().to_string())
        .filter(|t| !t.is_empty())
        .collect::<Vec<String>>()
}

fn dict_file() -> PathBuf {
    let mut dir = dirs::data_dir().unwrap();
    dir.push("jap_learn");
    if !dir.exists() {
        fs::create_dir(dir.clone()).unwrap();
    }
    dir.push("dict.json");
    dir
}
