use crate::lang::KanaSet;
use crate::Page::Back;
use crate::{NavigatedPage, Page};
use iced::widget::*;
use iced::{alignment, Element, Fill, Font};
use rand::seq::SliceRandom;

#[derive(Clone, Debug)]
pub struct WritingState {
    kana: String,
    set: Vec<(String, String)>,
    original_set: Vec<(String, String)>,
    kana_total: String,
    roman_total: String,
    is_help: bool,
    next_text: String,
}

impl NavigatedPage<WritingMessage> for WritingState {
    fn navigate(&self, message: &WritingMessage) -> Option<Page> {
        if let WritingMessage::Back = message {
            Some(Back)
        } else {
            None
        }
    }
}

impl WritingState {
    pub(crate) fn new(set: &KanaSet) -> WritingState {
        let mut list = set.list();
        list.shuffle(&mut rand::rng());
        WritingState {
            kana: "".to_string(),
            set: list.clone(),
            original_set: list,
            kana_total: "".to_string(),
            roman_total: "".to_string(),
            is_help: false,
            next_text: "Дальше".to_string(),
        }
    }
}

impl WritingState {
    pub fn update(&mut self, message: WritingMessage) {
        match message {
            WritingMessage::Back => todo!(),
            WritingMessage::Next => self.next(),
        }
    }

    fn next(&mut self) {
        if self.set.is_empty() {
            self.kana = "".to_string();
            self.roman_total = "".to_string();
            self.kana_total = "".to_string();

            self.set = self.original_set.clone();
            return;
        }

        let current = self.set.pop().unwrap();
        self.kana_total += &*format!("{} ", &current.0).to_string();
        self.roman_total += &*format!("{} ", &current.1.clone()).to_string();
        self.kana = current.1;
    }

    pub fn view(&self) -> Element<'_, WritingMessage> {
        container(
            iced::widget::column![
                text!("{}", self.roman_total).size(24),
                text!("{}", self.kana).size(48),
                self.answers(),
                row![
                    button(text!("{}", self.next_text)).on_press(WritingMessage::Next),
                    button("Закончить").on_press(WritingMessage::Back),
                ]
                .spacing(10)
            ]
            .spacing(10)
            .align_x(alignment::Horizontal::Center),
        )
        .center_x(Fill)
        .into()
    }

    fn answers(&self) -> Element<'_, WritingMessage> {
        if self.set.is_empty() {
            text!("{}", self.kana_total).font(Font::with_name("AppleGothic")).size(36).into()
        } else {
            vertical_space().height(36).into()
        }
    }
}
#[derive(Debug, Clone)]
pub enum WritingMessage {
    Next,
    Back,
}

#[derive(Default, Debug, Clone)]
pub struct Score {
    total: i32,
    correct: i32,
    fail: i32,
}
