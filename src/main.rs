mod lang;
mod quiz;
mod selector;
mod writing;

use crate::quiz::*;
use crate::selector::*;
use crate::Page::{Quiz, Selector, Writing};
use iced::widget::text;
use iced::Element;
use crate::writing::{WritingMessage, WritingState};

fn main() -> iced::Result {
    iced::application("A kana learn app", ScreenState::update, ScreenState::view).run()
}

#[derive(Debug, Clone)]
pub enum RootMessage {
    Selector(SelectorMessage),
    Quiz(QuizMessage),
    Writing(WritingMessage),
}

enum Page {
    Selector(SelectorState),
    Quiz(QuizState),
    Writing(WritingState),
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
        state_update!(message, self.stack, Selector, Quiz, Writing);
    }

    pub fn view(&self) -> Element<'_, RootMessage> {
        view_navigation!(self.stack, Quiz, Selector, Writing)
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
            if let Page::Back = new_page {
                $stack.pop();
                return;
            }
            $stack.push(new_page);
        } else {
            $state.update($msg);
        }
    };
}

trait NavigatedPage<T> {
    fn navigate(&self, message: &T) -> Option<Page>;
}
