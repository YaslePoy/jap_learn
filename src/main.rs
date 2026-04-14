#![windows_subsystem = "windows"]
mod dictionary;
mod dictionary_test;
mod lang;
mod quiz;
mod randomizer;
mod repetition;
mod repetitions;
mod selector;
mod writing;
mod data_provider;
use crate::data_provider::card_sets::load_sets;
use crate::data_provider::words::{create_db, load_words};
use crate::dictionary::{app_data_dir, DictionaryMessage, DictionaryState};
use crate::dictionary_test::{DictionaryQuizMessage, DictionaryQuizState};
use crate::lang::DictionaryElement;
use crate::quiz::*;
use crate::randomizer::randomizer::{RandomizerMessage, RandomizerState};
use crate::repetition::{RepetitionMessage, RepetitionState};
use crate::repetitions::{CardSetSettings, RepetitionsMessage, RepetitionsState};
use crate::selector::*;
use crate::writing::{WritingMessage, WritingState};
use crate::Page::{
    Dictionary, DictionaryQuiz, Quiz, Randomizer, Repetition, Repetitions, Selector, Writing,
};
use iced::widget::text;
use iced::{keyboard, Element, Program, Subscription};
use iced::{Font, Task};
use rusqlite::Connection;
use std::sync::{Arc, Mutex};
use iced::keyboard::Event;
use crate::RootMessage::Keyboard;

fn main() -> iced::Result {
    iced::application(ScreenState::boot, ScreenState::update, ScreenState::view).subscription(subscription)
        .title("Kana learn app")
        .font(include_bytes!("../noto.ttf"))
        .default_font(Font::with_name("Noto Sans JP"))
        .
            run()
}

fn subscription(_state: &ScreenState) -> Subscription<RootMessage> {
    keyboard::listen().map(|e| Keyboard(e))
}

#[derive(Clone)]
pub enum RootMessage {
    Selector(SelectorMessage),
    Quiz(QuizMessage),
    Writing(WritingMessage),
    Dictionary(DictionaryMessage),
    DictionaryQuiz(DictionaryQuizMessage),
    Randomizer(RandomizerMessage),
    Repetitions(RepetitionsMessage),
    Repetition(RepetitionMessage),
    Keyboard(keyboard::Event),
}

enum Page {
    Selector(SelectorState),
    Quiz(QuizState),
    Writing(WritingState),
    Dictionary(DictionaryState),
    DictionaryQuiz(DictionaryQuizState),
    Randomizer(RandomizerState),
    Repetitions(RepetitionsState),
    Repetition(RepetitionState),
    PreviousPage,
}

pub struct ScreenState {
    stack: Vec<Page>,
}

pub struct AppState {
    pub dictionary: Vec<DictionaryElement>,
    pub card_sets: Vec<CardSetSettings>,
    pub connection: Connection
}

impl Default for ScreenState {
    fn default() -> Self {
        let path = app_data_dir();
        let db_file = path.join("data.db");
        let connection = Connection::open(db_file).unwrap();
        let list: Vec<DictionaryElement> = load_words(&connection);
        let sets: Vec<CardSetSettings> = load_sets(&connection);

        let state = Arc::new(Mutex::new(AppState { dictionary: list, card_sets: sets, connection }));
        ScreenState {
            stack: vec![Selector(SelectorState::new(state.clone()))],
        }
    }
}

impl ScreenState {
    pub fn boot() -> (ScreenState, Task<RootMessage>) {
        create_db();
        (ScreenState::default(), Task::none())
    }
    pub fn update(&mut self, message: RootMessage) -> Task<RootMessage> {
        if let Keyboard(e) = message {
            return self.handle_keyboard(e);
        }

        state_update!(
            message,
            self.stack,
            Selector,
            Quiz,
            Writing,
            Dictionary,
            DictionaryQuiz,
            Randomizer,
            Repetitions,
            Repetition
        );
        Task::none()
    }
    pub fn view(&self) -> Element<'_, RootMessage> {
        view_navigation!(
            self.stack,
            Quiz,
            Selector,
            Writing,
            Dictionary,
            DictionaryQuiz,
            Randomizer,
            Repetitions,
            Repetition
        )
    }

    fn handle_keyboard(&mut self, message: Event) -> Task<RootMessage> {
        let page = self.stack.last_mut().unwrap();
        match page {
            Repetition(page) => page.press(&message),
            _ => {}
        }
        Task::none()
    }
}



#[macro_export]
macro_rules! view_navigation {
    ($stack:expr, $($e:ident), *) => {
        match &$stack.last().unwrap() {
            $(
            $e(s) => s.view().map(RootMessage::$e),
            )*
            _ => text!("").into(),
        }
    }
}

#[macro_export]
macro_rules! state_update {
    ($message:expr, $stack:expr, $($e:ident), *) => {
        match $message {
            $(
            RootMessage::$e(msg) => {
                if let $e(s) = $stack.last_mut().unwrap() {
                    message_navigation!(msg, $stack, s)
                }
            }
            )*
            _ => {}
        }
    }
}

#[macro_export]
macro_rules! message_navigation {
    ($msg:expr, $stack:expr, $state:expr) => {
        if let Some(new_page) = $state.navigate(&$msg) {
            if let Page::PreviousPage = new_page {
                $stack.pop();
                return Task::none();
            }
            $stack.push(new_page);
        } else {
            return $state.update($msg);
        }
    };
}

trait NavigatedPage<T> {
    fn navigate(&self, message: &T) -> Option<Page>;
}

pub trait KeyPressedPage {
    fn press(&mut self, message: &keyboard::Event);
}