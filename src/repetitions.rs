use crate::dictionary::DictionaryElement;
use crate::Page::PreviousPage;
use crate::{AppState, NavigatedPage, Page, RootMessage};
use iced::widget::button::{Catalog, Style};
use iced::widget::{button, column, container, row, scrollable, space, text, text_input, Column};
use iced::Background::Color;
use iced::{Border, Center, Element, Fill, Left, Length, Shadow, Task, Theme};
use rhai::{Engine, Scope};
use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub struct RepetitionsState {
    sets: Vec<CardSet>,
    selected_set: Option<usize>,
    correct_filters: Vec<bool>,
    pub state: Arc<Mutex<AppState>>,
}

impl NavigatedPage<RepetitionsMessage> for RepetitionsState {
    fn navigate(&self, message: &RepetitionsMessage) -> Option<Page> {
        if let RepetitionsMessage::Back = message {
            Some(PreviousPage)
        } else {
            None
        }
    }
}

impl RepetitionsState {
    pub(crate) fn new(state: Arc<Mutex<AppState>>) -> RepetitionsState {
        RepetitionsState {
            sets: Vec::new(),
            selected_set: None,
            correct_filters: vec![],
            state,
        }
    }
}

impl RepetitionsState {
    pub fn update(&mut self, message: RepetitionsMessage) -> Task<RootMessage> {
        match message {
            RepetitionsMessage::Next => {}
            RepetitionsMessage::Back => {}
            RepetitionsMessage::GoToRepetition => {}
            RepetitionsMessage::CreateSet => {
                self.sets.push(CardSet::with_name(format!(
                    "Card set #{}",
                    self.sets.len() + 1
                )));
                self.correct_filters.push(true);
            }
            RepetitionsMessage::DeleteSet => {
                self.sets.remove(self.selected_set.unwrap());
                self.correct_filters.remove(self.selected_set.unwrap());
                self.selected_set = None;
            }
            RepetitionsMessage::SelectSet(index) => {
                self.selected_set = Some(index);
            }
            RepetitionsMessage::SetName(new) => {
                self.sets[self.selected_set.unwrap()].name = new;
            }
            RepetitionsMessage::Save => {
                for i in 0..self.sets.len() {
                    self.correct_filters[i] = self.sets[i].check_filter();
                }

                if self.correct_filters.iter().all(|x| *x) {
                    println!("Saving");
                } else {
                    println!("Some errors");
                }
            }
            RepetitionsMessage::SetForward(new) => {
                self.sets[self.selected_set.unwrap()].forward = new;
            }
            RepetitionsMessage::SetBackward(new) => {
                self.sets[self.selected_set.unwrap()].backward = new;
            }
            RepetitionsMessage::SetFilter(new) => {
                self.sets[self.selected_set.unwrap()].filter = new;
            }
            RepetitionsMessage::TryFilter => {
                let set = &mut self.sets[self.selected_set.unwrap()];
                let count = set.get_word_list(&self.state.lock().unwrap()).len();
                set.count = Some(count);
            }
        }
        Task::none()
    }

    pub fn view(&self) -> Element<'_, RepetitionsMessage> {
        container(
            iced::widget::column![
                button("Назад").on_press(RepetitionsMessage::Back),
                row![
                    column![
                        scrollable(self.sets_list()).height(Fill),
                        button("Добавить")
                            .width(Fill)
                            .on_press(RepetitionsMessage::CreateSet),
                        button("Сохранить")
                            .width(Fill)
                            .on_press(RepetitionsMessage::Save),
                    ]
                    .spacing(10)
                    .width(Length::FillPortion(1)),
                    self.selected_set_view(),
                    self.launch_button()
                ]
                .align_y(Center)
                .padding(10)
                .spacing(10)
                .width(Fill)
                .height(Fill)
            ]
                .align_x(Left)
                .width(Fill),
        )
            .center_x(Fill)
            .padding(10)
            .into()
    }

    fn launch_button(&self) -> Element<'_, RepetitionsMessage> {
        if let Some(_) = self.selected_set {
            return button(text!("▷").height(Fill).center())
                .height(200)
                .on_press(RepetitionsMessage::GoToRepetition)
                .into();
        }
        space().into()
    }

    fn selected_set_view(&self) -> Element<'_, RepetitionsMessage> {
        if let Some(index) = self.selected_set {
            return column![
                scrollable(
                    column![
                        text_input("Название набора", &self.sets[index].name)
                            .on_input(RepetitionsMessage::SetName),
                        text_input("Передняя сторона", &self.sets[index].forward)
                            .on_input(RepetitionsMessage::SetForward),
                        text_input("Задняя сторона", &self.sets[index].backward)
                            .on_input(RepetitionsMessage::SetBackward),
                        text!("Фильтр"),
                        text_input("", &self.sets[index].filter)
                            .on_input(RepetitionsMessage::SetFilter),
                        button("Проверить фильтр").on_press(RepetitionsMessage::TryFilter),
                        self.count_view()
                    ]
                    .spacing(10)
                )
                .height(Fill),
                button("Удалить")
                    .style(|x: &Theme, _status| Style {
                        background: Some(Color(x.palette().danger)),
                        text_color: x.palette().text,
                        border: Default::default(),
                        shadow: Default::default(),
                        snap: false,
                    })
                    .on_press(RepetitionsMessage::DeleteSet),
            ]
                .spacing(10)
                .width(Length::FillPortion(2))
                .into();
        }
        space().width(Length::FillPortion(2)).into()
    }

    fn count_view(&self) -> Element<'_, RepetitionsMessage> {
        if let Some(count) = self.sets[self.selected_set.unwrap()].count {
            return text!("Колличество слов: {}", count).into();
        }
        space().into()
    }

    fn sets_list(&self) -> Column<'_, RepetitionsMessage> {
        let mut column = Column::new();
        let mut i = 0;
        for set in &self.sets {
            column = column.push(
                button(text!("{}", set.name.clone()))
                    .on_press_with(move || RepetitionsMessage::SelectSet(i.clone()))
                    .style(move |_x: &Theme, _status| Style {
                        background: None,
                        text_color: if self.correct_filters[i.clone()] {
                            _x.palette().text
                        } else {
                            _x.palette().warning
                        },
                        border: Border::default(),
                        shadow: Shadow::default(),
                        snap: false,
                    }),
            );
            i += 1;
        }

        column
    }
}
#[derive(Debug, Clone)]
pub enum RepetitionsMessage {
    Next,
    Back,
    GoToRepetition,
    CreateSet,
    DeleteSet,
    SetName(String),
    SelectSet(usize),
    Save,
    SetForward(String),
    SetBackward(String),
    SetFilter(String),
    TryFilter,
}

#[derive(Debug, Clone)]
pub struct CardSet {
    name: String,
    forward: String,
    backward: String,
    filter: String,
    count: Option<usize>,
}

impl CardSet {
    fn with_name(name: String) -> CardSet {
        CardSet {
            name,
            forward: "".to_string(),
            backward: "".to_string(),
            filter: "true".to_string(),
            count: None,
        }
    }

    fn check_filter(&self) -> bool {
        let engine = Engine::new();
        let ast = engine.compile(&self.filter);
        ast.is_ok()
    }

    pub fn get_word_list(&self, state: &AppState) -> Vec<DictionaryElement> {
        let mut list = vec![];
        let engine = Engine::new();
        let ast = engine.compile(&self.filter);
        if ast.is_err() {
            return list;
        }

        let ast = ast.unwrap();

        for word in &state.dictionary {
            let mut more = rhai::Map::new();
            for iced in &word.additional {
                more.insert(iced.0.clone().into(), iced.1.clone().into());
            }
            let mut scope = Scope::new();
            scope.push_constant("key", word.key.clone())
                .push_constant("value", word.value.clone())
                .push_constant("tags", word.tags.clone())
                .push_constant("more", more);

            let result = engine.eval_ast_with_scope::<bool>(&mut scope, &ast);
            if result.is_ok() && result.unwrap() {
                list.push(word.clone());
            }
        }

        list
    }
}
