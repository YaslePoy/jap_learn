use crate::dictionary::{split_with_coma, DictionaryElement};
use crate::quiz::Score;
use crate::Page::PreviousPage;
use crate::RootMessage;
use crate::{NavigatedPage, Page};
use iced::border::Radius;
use iced::widget::container::Style;
use iced::widget::{button, container, row, space, text, text_input, Row};
use iced::Background::Color;
use iced::{alignment, Border, Element, Fill, Task, Theme};
use rand::prelude::SliceRandom;
#[derive(Debug, Clone)]
pub struct DictionaryQuizState {
    words: Vec<DictionaryElement>,
    current_set: Vec<DictionaryElement>,
    answer: String,
    view: String,
    correct: String,
    score: Score,
    is_help: bool,
    reverse: bool,
    laps: u32
}
#[derive(Debug, Clone)]
pub enum DictionaryQuizMessage {
    Next,
    Back,
    AnswerChanged(String),
    SubmitAnswer,
}

impl NavigatedPage<DictionaryQuizMessage> for DictionaryQuizState {
    fn navigate(&self, message: &DictionaryQuizMessage) -> Option<Page> {
        match message {
            DictionaryQuizMessage::Back => Some(PreviousPage),
            _ => None,
        }
    }
}

impl DictionaryQuizState {
    pub fn new(words: Vec<DictionaryElement>, reverse: bool) -> DictionaryQuizState {
        DictionaryQuizState {
            words,
            current_set: Vec::new(),
            answer: "".to_string(),
            view: "---".to_string(),
            correct: "".to_string(),
            score: Default::default(),
            is_help: false,
            reverse,
            laps: 0
        }
    }

    pub fn update(&mut self, message: DictionaryQuizMessage) -> Task<RootMessage> {
        match message {
            DictionaryQuizMessage::Next => {
                self.next();
            }
            DictionaryQuizMessage::Back => {}
            DictionaryQuizMessage::AnswerChanged(c) => self.answer = c.clone(),
            DictionaryQuizMessage::SubmitAnswer => self.submit(),
        }
        Task::none()
    }

    pub fn view(&self) -> Element<'_, DictionaryQuizMessage> {
        container(
            iced::widget::column![
                self.laps(),
                row![
                    text!("{}", self.view).size(54),
                    text!(
                        "{}",
                        if self.is_help {
                            self.correct.clone()
                        } else {
                            String::new()
                        }
                    ),
                ]
                .align_y(alignment::Vertical::Center)
                .spacing(20),
                text_input("Перевод", &self.answer)
                    .size(28)
                    .width(250)
                    .on_input(DictionaryQuizMessage::AnswerChanged)
                    .on_submit(DictionaryQuizMessage::SubmitAnswer),
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
                button("Закончить").on_press(DictionaryQuizMessage::Back),
            ]
            .spacing(10)
            .align_x(alignment::Horizontal::Center),
        )
        .center_y(Fill)
        .center_x(Fill)
        .into()
    }
    fn next(&mut self) {}

    fn submit(&mut self) {
        if self.view == "---" {
            self.show_next();
            return;
        }

        if !self.is_help {
            self.score.total += 1;
        }
        if self.answer == self.correct || split_with_coma(self.correct.clone()).contains(&self.answer) {
            if self.is_help == false {
                self.score.correct += 1;
            }
            self.show_next()
        } else {

            self.score.fail += 1;
            self.is_help = true;
        }
    }

    fn update_set(&mut self) {
        self.current_set.append(&mut self.words.clone());
        self.current_set.shuffle(&mut rand::rng());
        if self.score.total != 0 {
            self.laps += 1;
        }
    }

    fn show_next(&mut self) {
        self.is_help = false;
        self.answer = String::new();

        if self.current_set.is_empty() {
            self.update_set();
        }

        let next = self.current_set.pop().unwrap();
        if self.reverse {
            self.view = next.value.clone();
            self.correct = next.key.clone();
        } else {
            self.view = next.key.clone();
            self.correct = next.value.clone();
        }
    }

    fn laps(&self) -> Element<'_, DictionaryQuizMessage> {
        let mut col = Row::new();
        for _ in 0..self.laps {
            col = col.push(iced::widget::container(space().height(15).width(15)).style(|x: &Theme| {
                Style{
                    text_color: None,
                    background: Some(Color(x.palette().primary)),
                    border: Border{
                        color: Default::default(),
                        width: 0.0,
                        radius: Radius::new(5),
                    },
                    shadow: Default::default(),
                    snap: false,
                }
            }));
        }
        col.spacing(10).into()
    }
}
