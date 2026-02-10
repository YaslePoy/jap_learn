mod lang;

use crate::Page::{Back, Quiz, Selector};
use iced::Font;
use iced::widget::{Column, Row, button, checkbox, column, container, row, text, text_input};
use iced::{Element, Fill, alignment};
use rand::RngExt;
use crate::lang::*;

fn main() -> iced::Result {
    iced::application("A kana learn app", ScreenState::update, ScreenState::view).run()
}

#[derive(Debug, Clone)]
pub enum RootMessage {
    Selector(SelectorMessage),
    Quiz(QuizMessage),
}

enum Page {
    Selector(SelectorState),
    Quiz(QuizState),
    Back,
}

impl Default for Page {
    fn default() -> Self {
        Selector(SelectorState::default())
    }
}

pub struct ScreenState {
    stack: Vec<Page>,
}

impl Default for ScreenState {
    fn default() -> Self {
        ScreenState {
            stack: vec![Page::default()],
        }
    }
}

impl ScreenState {
    pub fn update(&mut self, message: RootMessage) {
        match message {
            RootMessage::Selector(selector) => {
                if let Selector(s) = self.stack.last_mut().unwrap() {
                    if let Some(new_page) = s.navigate(&selector) {
                        if let Page::Back = new_page {
                            self.stack.pop();
                            return;
                        }
                        self.stack.push(new_page);
                    } else {
                        s.update(selector);
                    }
                }
            }

            RootMessage::Quiz(quiz) => {
                if let Page::Quiz(q) = self.stack.last_mut().unwrap() {
                    if let Some(new_page) = q.navigate(&quiz) {
                        if let Page::Back = new_page {
                            self.stack.pop();
                            return;
                        }
                        self.stack.push(new_page);
                    } else {
                        q.update(quiz);
                    }
                }
            }
        }
    }

    pub fn view(&self) -> Element<'_, RootMessage> {
        match &self.stack.last().unwrap() {
            Quiz(q) => q.view().map(RootMessage::Quiz),
            Selector(s) => s.view().map(RootMessage::Selector),
            _ => text!("").into(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum QuizMessage {
    ContentChanged(String),
    Back,
}



#[derive(Default, Debug, Clone)]
pub struct Score {
    total: i32,
    correct: i32,
    fail: i32,
}

trait NavigatedPage<T> {
    fn navigate(&self, message: &T) -> Option<Page>;
}

#[derive(Clone, Debug)]
struct QuizState {
    kana: String,
    current_romanji: String,
    correct_roman: String,
    set: KanaSet,
    score: Score,
    is_help: bool,
}

impl NavigatedPage<QuizMessage> for QuizState {
    fn navigate(&self, message: &QuizMessage) -> Option<Page> {
        if let QuizMessage::Back = message {
            Some(Back)
        } else {
            None
        }
    }
}

impl QuizState {
    fn new() -> QuizState {
        QuizState {
            kana: "ぁ".to_string(),
            correct_roman: "a".to_string(),
            is_help: false,
            current_romanji: "".to_string(),
            set: KanaSet::hiragana(),
            score: Score {
                total: 0,
                correct: 0,
                fail: 0,
            },
        }
    }
}

impl Default for QuizState {
    fn default() -> Self {
        QuizState::new()
    }
}

impl QuizState {
    pub fn update(&mut self, message: QuizMessage) {
        match message {
            QuizMessage::ContentChanged(content) => {
                if content.contains("`") {
                    self.is_help = true;
                    self.score.fail += 1;

                    return;
                }
                self.current_romanji = content;
                println!("Content: {}", self.current_romanji);
                if self.correct_roman == self.current_romanji {
                    if self.is_help == false {
                        self.score.correct += 1;
                    }

                    self.is_help = false;
                    self.score.total += 1;
                    self.update_showed()
                }
            }
            QuizMessage::Back => todo!(),
        }
    }

    fn update_showed(&mut self) {
        self.current_romanji = String::new();
        let pair = self.set.next();
        self.kana = pair.0;
        self.correct_roman = pair.1;
    }

    pub fn view(&self) -> Element<'_, QuizMessage> {
        container(
            column![
                row![
                    text!("{}", self.kana.to_uppercase())
                        .size(48)
                        .font(Font::with_name("AppleGothic")),
                    text!(
                        "{}",
                        if self.is_help {
                            self.correct_roman.clone()
                        } else {
                            String::new()
                        }
                    ),
                ]
                .align_y(alignment::Vertical::Center)
                .spacing(20),
                text_input("Романдзи", &self.current_romanji)
                    .size(28)
                    .width(150)
                    .on_input(QuizMessage::ContentChanged),
                row![
                    text!("{}", self.score.total.to_string()).size(25),
                    text!("{}", self.score.correct.to_string())
                        .size(25)
                        .color(iced::Color::from_rgb8(60, 170, 60)),
                    text!("{}", self.score.fail.to_string())
                        .color(iced::Color::from_rgb8(255, 79, 0))
                        .size(25),
                ]
                .spacing(10),
                button("Закончить").on_press(QuizMessage::Back),
            ]
            .spacing(10)
            .align_x(alignment::Horizontal::Center),
        )
        .center_x(Fill)
        .into()
    }
}

pub struct SelectorState {
    pub set: KanaSet,
}

impl Default for SelectorState {
    fn default() -> Self {
        Self {
            set: KanaSet::hiragana(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum SelectorMessage {
    Change,
    Goto,
    Check(usize, bool),
}

impl NavigatedPage<SelectorMessage> for SelectorState {
    fn navigate(&self, message: &SelectorMessage) -> Option<Page> {
        if let SelectorMessage::Goto = message {
            let mut quiz = QuizState::new();
            quiz.set = self.set.clone();
            return Some(Quiz(quiz));
        }
        None
    }
}

impl SelectorState {
    pub fn update(&mut self, message: SelectorMessage) {
        match message {
            SelectorMessage::Change => match self.set.chars_type {
                KanaType::Katakana => self.set = KanaSet::hiragana(),
                KanaType::Hiragana => self.set = KanaSet::katakana(),
            },
            SelectorMessage::Goto => {}
            SelectorMessage::Check(i, b) => self.set.include_map[i] = b,
        }
    }

    pub fn view(&self) -> Element<'_, SelectorMessage> {
        container(
            column![
                button("Переключить").on_press(SelectorMessage::Change),
                self.rows_selector(),
                button("К тесту").on_press(SelectorMessage::Goto),
            ]
            .spacing(10),
        )
        .padding(20)
        .into()
    }

    fn rows_selector(&self) -> Element<'_, SelectorMessage> {
        let mut row = Row::new();

        for i in 0..self.set.dictionary.len() {
            let setup_checked = move |b: bool| -> SelectorMessage { SelectorMessage::Check(i, b) };

            let mut chars_column: Column<'_, _> = Column::new();
            chars_column =
                chars_column.push(checkbox("", self.set.include_map[i]).on_toggle(setup_checked));

            for v in &self.set.dictionary[i] {
                chars_column = chars_column.push(
                    container(
                        text!("{}", v.0.clone().to_uppercase())
                            .size(36)
                            .font(Font::with_name("AppleGothic")),
                    )
                    .padding(20)
                    .style(container::rounded_box),
                );
            }

            row = row.push(
                chars_column
                    .spacing(10)
                    .align_x(alignment::Horizontal::Center),
            );
        }

        row.spacing(10).into()
    }
}
