use crate::lang::{KanaSet, KanaType};
use crate::Page::Quiz;
use crate::{NavigatedPage, Page, QuizState};
use iced::widget::{button, checkbox, container, text, Column, Row};
use iced::{alignment, Element, Font};

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
            iced::widget::column![
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
