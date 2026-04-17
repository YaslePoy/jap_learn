use crate::lang::DictionaryElement;
use crate::Page::PreviousPage;
use crate::{AppState, NavigatedPage, Page, RootMessage, DEFAULT_SPACING};
use iced::widget::button::danger;
use iced::widget::{button, column, container, row, text, text_input};
use iced::{Element, Fill, Task};
use std::sync::{Arc, Mutex};
use crate::data_provider::words::{delete_word, update_word};

#[derive(Clone)]
pub struct WordState {
    state: Arc<Mutex<AppState>>,
    index: usize,
    word: DictionaryElement,
}

impl NavigatedPage<WordMessage> for WordState {
    fn navigate(&self, message: &WordMessage) -> Option<Page> {
        if let WordMessage::Back = message {
            Some(PreviousPage)
        } else {
            None
        }
    }
}

impl WordState {
    pub(crate) fn new(
        word: DictionaryElement,
        index: usize,
        state: Arc<Mutex<AppState>>,
    ) -> WordState {
        WordState { state, index, word }
    }
}

impl WordState {
    pub fn update(&mut self, message: WordMessage) -> Task<RootMessage> {
        match message {
            WordMessage::Back => {},
            WordMessage::Save => {
                let mut state = self.state.lock().unwrap();
                state.dictionary[self.index] = self.word.clone();
                update_word(&mut self.word, &state.connection)
            }
            WordMessage::Delete => {
                let mut state = self.state.lock().unwrap();
                state.dictionary.remove(self.index);
                delete_word(&self.word, &state.connection);
                return Task::done(RootMessage::Word(WordMessage::Back))
            }
            WordMessage::SetTags(n) => self.word.tags = n,
            WordMessage::SetKey(n) => {
                self.word.key = n;
            }
            WordMessage::SetValue(n) => {
                self.word.value = n;
            }
        }
        Task::none()
    }

    pub fn view(&self) -> Element<'_, WordMessage> {
        container(
            column![
                iced::widget::column![
                    button("Назад").on_press(WordMessage::Back),
                    text!("Ключ"),
                    text_input("key", &self.word.key).on_input(WordMessage::SetKey),
                    text!("Значение"),
                    text_input("value", &self.word.value).on_input(WordMessage::SetValue),
                    text!("Теги"),
                    text_input("tags", &self.word.tags).on_input(WordMessage::SetTags),
                ]
                .spacing(DEFAULT_SPACING)
                .width(Fill)
                .height(Fill),
                row![
                    button("Сохранить").on_press(WordMessage::Save),
                    button("Удалить")
                        .style(danger)
                        .on_press(WordMessage::Delete),
                ]
                .spacing(DEFAULT_SPACING)
            ]
            .spacing(DEFAULT_SPACING),
        )
        .padding(DEFAULT_SPACING)
        .into()
    }
}
#[derive(Debug, Clone)]
pub enum WordMessage {
    Save,
    Back,
    Delete,
    SetTags(String),
    SetKey(String),
    SetValue(String),
}
