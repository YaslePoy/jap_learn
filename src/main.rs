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

use std::fs::File;
use std::io::Read;
use crate::dictionary::{DictionaryElement, DictionaryMessage, DictionaryState};
use crate::dictionary_test::{DictionaryQuizMessage, DictionaryQuizState};
use crate::quiz::*;
use crate::randomizer::randomizer::{RandomizerMessage, RandomizerState};
use crate::repetition::{RepetitionMessage, RepetitionState};
use crate::repetitions::{RepetitionsMessage, RepetitionsState};
use crate::selector::*;
use crate::writing::{WritingMessage, WritingState};
use crate::Page::{
    Dictionary, DictionaryQuiz, Quiz, Randomizer, Repetition, Repetitions, Selector, Writing,
};
use iced::widget::text;
use iced::Element;
use iced::{Font, Task};
use std::sync::{Arc, Mutex};

fn main() -> iced::Result {
    iced::application(ScreenState::boot, ScreenState::update, ScreenState::view)
        .title("Kana learn app")
        .font(include_bytes!("../noto.ttf"))
        .default_font(Font::with_name("Noto Sans JP"))
        .run()
}

#[derive(Debug, Clone)]
pub enum RootMessage {
    Selector(SelectorMessage),
    Quiz(QuizMessage),
    Writing(WritingMessage),
    Dictionary(DictionaryMessage),
    DictionaryQuiz(DictionaryQuizMessage),
    Randomizer(RandomizerMessage),
    Repetitions(RepetitionsMessage),
    Repetition(RepetitionMessage),
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
}

impl Default for ScreenState {
    fn default() -> Self {
        let mut current_dict = "[]".to_string();
        match File::open(dictionary::dict_file()) {
            Ok(mut f) => {
                current_dict = String::new();
                f.read_to_string(&mut current_dict).unwrap();
            }
            _ => {}
        }
        
        let list: Vec<DictionaryElement> = serde_json::from_str(&current_dict).unwrap();
        let state = Arc::new(Mutex::new(AppState { dictionary: list }));
        ScreenState {
            stack: vec![Selector(SelectorState::new(state.clone()))],
        }
    }
}

impl ScreenState {
    pub fn boot() -> (ScreenState, Task<RootMessage>) {
        (ScreenState::default(), Task::none())
    }
    pub fn update(&mut self, message: RootMessage) -> Task<RootMessage> {
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
