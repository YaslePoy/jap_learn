#![windows_subsystem = "windows"]
mod lang;
mod quiz;
mod selector;
mod writing;
mod dictionary;
mod dictionary_test;
mod randomizer;

use crate::quiz::*;
use crate::selector::*;
use crate::writing::{WritingMessage, WritingState};
use crate::Page::{Dictionary, DictionaryQuiz, Quiz, Randomizer, Selector, Writing};
use iced::widget::text;
use iced::{Font, Task};
use iced::Element;
use crate::dictionary::{DictionaryMessage, DictionaryState};
use crate::dictionary_test::{DictionaryQuizMessage, DictionaryQuizState};
use crate::randomizer::randomizer::{ RandomizerMessage, RandomizerState};

fn main() -> iced::Result {
    iced::application(ScreenState::boot, ScreenState::update, ScreenState::view)
        .title("Kana learn app").font(include_bytes!("../noto.ttf")).default_font(Font::with_name("Noto Sans JP")).run()
}

#[derive(Debug, Clone)]
pub enum RootMessage {
    Selector(SelectorMessage),
    Quiz(QuizMessage),
    Writing(WritingMessage),
    Dictionary(DictionaryMessage),
    DictionaryQuiz(DictionaryQuizMessage),
    Randomizer(RandomizerMessage)
}

enum Page {
    Selector(SelectorState),
    Quiz(QuizState),
    Writing(WritingState),
    Dictionary(DictionaryState),
    DictionaryQuiz(DictionaryQuizState),
    Randomizer(RandomizerState),
    PreviousPage,
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
    pub fn boot() -> (ScreenState, Task<RootMessage>){
        (ScreenState::default(), Task::none())
    }
    pub fn update(&mut self, message: RootMessage)  -> Task<RootMessage> {
        state_update!(message, self.stack, Selector, Quiz, Writing, Dictionary, DictionaryQuiz, Randomizer);
        Task::none()
    }

    pub fn view(&self) -> Element<'_, RootMessage> {
        view_navigation!(self.stack, Quiz, Selector, Writing, Dictionary, DictionaryQuiz, Randomizer)
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