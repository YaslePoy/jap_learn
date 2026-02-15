use crate::lang::KanaSet;
use crate::Page::PreviousPage;
use crate::{NavigatedPage, Page};
use iced::widget::*;
use iced::{alignment, Element, Fill};
use rand::seq::SliceRandom;

#[derive(Clone, Debug)]
pub struct QuizState {
    kana: String,
    current_roman: String,
    correct_roman: String,
    pub(crate) set: KanaSet,
    queue: Vec<(String, String)>,
    score: Score,
    is_help: bool,
}

impl NavigatedPage<QuizMessage> for QuizState {
    fn navigate(&self, message: &QuizMessage) -> Option<Page> {
        if let QuizMessage::Back = message {
            Some(PreviousPage)
        } else {
            None
        }
    }
}

impl QuizState {
    pub(crate) fn new() -> QuizState {
        QuizState {
            kana: "ぁ".to_string(),
            correct_roman: "a".to_string(),
            is_help: false,
            current_roman: "".to_string(),
            set: KanaSet::hiragana(),
            queue: Vec::new(),
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
                self.current_roman = content;
                if self.correct_roman == self.current_roman {
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
        self.current_roman = String::new();
        let queue = &mut self.queue;

        if queue.is_empty() {
            for pair in self.set.list() {
                queue.push(pair);
            }

            queue.shuffle(&mut rand::rng());
        }

        let pair = self.queue.pop().unwrap();
        self.kana = pair.0;
        self.correct_roman = pair.1;
    }

    pub fn view(&self) -> Element<'_, QuizMessage> {
        container(
            iced::widget::column![
                row![
                    text!("{}", self.kana.to_uppercase())
                        .size(54),
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
                text_input("Романдзи", &self.current_roman)
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
            .center_y(Fill)
        .center_x(Fill)
        .into()
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
