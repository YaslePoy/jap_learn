use crate::lang::{CardSet, DictionaryElement, WordOpenMode};
use crate::repetitions::CardSetSettings;
use crate::Page::PreviousPage;
use crate::{AppState, KeyPressedPage, NavigatedPage, Page, RootMessage};
use iced::alignment::Horizontal::Center;
use iced::keyboard::key::Physical::Code;
use iced::widget::container::rounded_box;
use iced::widget::space::vertical;
use iced::widget::{button, column, container, row, space, text};
use iced::{alignment, keyboard, Element, Fill, Left, Task};
use std::sync::{Arc, Mutex};

pub struct RepetitionState {
    pub settings: CardSetSettings,
    pub set: CardSet,
    pub state: Arc<Mutex<AppState>>,
    current_word: DictionaryElement,
    open: bool,
}

impl NavigatedPage<RepetitionMessage> for RepetitionState {
    fn navigate(&self, message: &RepetitionMessage) -> Option<Page> {
        if let RepetitionMessage::Back = message {
            Some(PreviousPage)
        } else {
            None
        }
    }
}

impl RepetitionState {
    pub(crate) fn new(set: CardSetSettings, state: Arc<Mutex<AppState>>) -> RepetitionState {
        let mut card_set = CardSet::new(&set, state.clone());
        let word = card_set.next();
        RepetitionState {
            settings: set,
            set: card_set,
            state,
            current_word: word,
            open: false,
        }
    }
}

impl RepetitionState {
    pub fn update(&mut self, message: RepetitionMessage) -> Task<RootMessage> {
        match message {
            RepetitionMessage::Back => {}
            RepetitionMessage::Next => {self.next()}
            RepetitionMessage::Answer(m) => {self.answer(m)}
        }

        Task::none()
    }

    fn next(&mut self) {
        if self.open {
            self.set.open(WordOpenMode::None);
            self.current_word = self.set.next();
            self.open = false;
            return;
        } else {
            self.open = true;
        }
    }

    fn answer(&mut self, mode: WordOpenMode) {
        if !self.open {
            return;
        }

        self.set.open(mode);
        self.open = false;
        self.current_word = self.set.next();
    }

    pub fn view(&self) -> Element<'_, RepetitionMessage> {
        container(
            iced::widget::column![
                button("Назад").on_press(RepetitionMessage::Back),
                column![
                    container(self.draw_forward())
                        .width(Fill)
                        .height(Fill)
                        .align_x(Center)
                        .align_y(alignment::Vertical::Center),
                    container(vertical().height(5)).width(Fill).padding(5).style(rounded_box),
                    container(self.draw_backward())
                        .width(Fill)
                        .height(Fill)
                        .align_x(Center)
                        .align_y(alignment::Vertical::Center),
                    container(
                    self.answer_bar()
                    ).width(Fill).align_x(Center).height(30)
                ]
                .height(Fill)
                .width(Fill)
            ]
            .align_x(Left)
            .width(Fill),
        )
        .center_x(Fill)
        .padding(10)
        .into()
    }

    fn draw_forward(&self) -> Element<'_, RepetitionMessage> {
        let word = &self.current_word;
        match self.settings.forward.as_str() {
            "key" => self.draw_key(word),
            "value" => self.draw_value(word),
            _ => space().into(),
        }

    }

    fn draw_backward(&self) -> Element<'_, RepetitionMessage> {
        if !self.open {
            return space().into();
        }

        let word = &self.current_word;
        match self.settings.backward.as_str() {
            "key" => self.draw_key(word),
            "value" => self.draw_value(word),
            _ => space().into(),
        }
    }

    fn answer_bar(&self) -> Element<'_, RepetitionMessage> {
        if !self.open {
            return space().into();
        }

        row![
            button("Не получилось").on_press(RepetitionMessage::Answer(WordOpenMode::None)),
            button("Трудно").on_press(RepetitionMessage::Answer(WordOpenMode::Hard)),
            button("Нормально").on_press(RepetitionMessage::Answer(WordOpenMode::Ok)),
            button("Легко").on_press(RepetitionMessage::Answer(WordOpenMode::Easy)),
        ]
        .spacing(10)
        .into()
    }

    fn draw_key(&self, word: &DictionaryElement) -> Element<'_, RepetitionMessage> {
        text!("{}", word.key).size(36).into()
    }
    fn draw_value(&self, word: &DictionaryElement) -> Element<'_, RepetitionMessage> {
        text!("{}", word.value).size(24).into()
    }
}

impl KeyPressedPage for RepetitionState {
    fn press(&mut self, message: &keyboard::Event) {
        if let keyboard::Event::KeyPressed {
            key: _,
            modified_key: _,
            physical_key: pk,
            location: _,
            modifiers: _,
            text: _,
            repeat: _,
        } = message
        {
            if let Code(code) = pk {
                match code {
                    keyboard::key::Code::Space => self.next(),
                    keyboard::key::Code::Digit1 => self.answer(WordOpenMode::None),
                    keyboard::key::Code::Digit2 => self.answer(WordOpenMode::Hard),
                    keyboard::key::Code::Digit3 => self.answer(WordOpenMode::Ok),
                    keyboard::key::Code::Digit4 => self.answer(WordOpenMode::Easy),
                    _ => {}
                }
            }
        }
    }
}

#[derive(Clone)]
pub enum RepetitionMessage {
    Next,
    Back,
    Answer(WordOpenMode),
}
