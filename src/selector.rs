use crate::lang::{KanaSet, KanaType};
use crate::selector::SelectorMessage::ChangeMode;
use crate::writing::WritingState;
use crate::Page::{Quiz, Writing};
use crate::{NavigatedPage, Page, QuizState};
use iced::widget::*;
use iced::{alignment, Element};

pub struct SelectorState {
    pub set: KanaSet,
    is_writing: bool,
}

impl Default for SelectorState {
    fn default() -> Self {
        Self {
            set: KanaSet::hiragana(),
            is_writing: false,
        }
    }
}

#[derive(Debug, Clone)]
pub enum SelectorMessage {
    Change,
    Goto,
    Check(usize, bool),
    ChangeMode(bool),
}

impl NavigatedPage<SelectorMessage> for SelectorState {
    fn navigate(&self, message: &SelectorMessage) -> Option<Page> {
        if let SelectorMessage::Goto = message {
            return if self.is_writing {
                let writing = WritingState::new(&self.set);
                Some(Writing(writing))
            } else {
                let mut quiz = QuizState::new();
                quiz.set = self.set.clone();
                Some(Quiz(quiz))
            };
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
            ChangeMode(b) => self.is_writing = b,
        }
    }

    pub fn view(&self) -> Element<'_, SelectorMessage> {
        container(
            iced::widget::column![
                button("Переключить").on_press(SelectorMessage::Change),
                self.rows_selector(),
                toggler(self.is_writing)
                    .label("Режим письма")
                    .on_toggle(ChangeMode),
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
                chars_column.push(checkbox(self.set.include_map[i]).on_toggle(setup_checked));

            for v in &self.set.dictionary[i] {
                chars_column = chars_column.push(
                    container(
                        text!("{}", v.0.clone().to_uppercase())
                            .size(36),
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
