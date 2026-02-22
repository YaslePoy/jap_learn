use crate::lang::KanaSet;
use crate::Page::PreviousPage;
use crate::{NavigatedPage, Page, RootMessage};
use iced::widget::*;
use iced::{alignment, Element, Fill, Task};
use rand::seq::SliceRandom;

#[derive(Clone, Debug)]
pub struct WritingState {
    kana: String,
    set: Vec<(String, String)>,
    original_set: Vec<(String, String)>,
    kana_total: String,
    roman_total: String,
    next_text: String,
    show_all: bool,
}

impl NavigatedPage<WritingMessage> for WritingState {
    fn navigate(&self, message: &WritingMessage) -> Option<Page> {
        if let WritingMessage::Back = message {
            Some(PreviousPage)
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
            next_text: "Дальше".to_string(),
            show_all: false,
        }
    }
}

impl WritingState {
    pub fn update(&mut self, message: WritingMessage) -> Task<RootMessage>  {
        match message {
            WritingMessage::Back => todo!(),
            WritingMessage::Next => self.next(),
            WritingMessage::SwitchShowMode(b) => self.show_all = b,
        }
        Task::none()
    }

    fn next(&mut self) {
        if self.set.is_empty() {
            self.kana = "".to_string();
            self.roman_total = "".to_string();
            self.kana_total = "".to_string();

            self.set = self.original_set.clone();
            self.set.shuffle(&mut rand::rng());
            return;
        }

        if self.show_all {

            if self.set.is_empty() == false && self.kana_total.is_empty() == false {
                self.set.clear();
            }
            for pair in &self.set {
                self.kana = "---".to_string();
                self.roman_total += &*format!("{} ", &pair.1.clone()).to_string();
                self.kana_total += &*format!("{} ", &pair.0).to_string();
            }


        } else {
            let current = self.set.pop().unwrap();
            self.kana_total += &*format!("{} ", &current.0).to_string();
            self.roman_total += &*format!("{} ", &current.1.clone()).to_string();
            self.kana = current.1;
        }
    }

    pub fn view(&self) -> Element<'_, WritingMessage> {
        container(
            iced::widget::column![
                checkbox(self.show_all)
                    .label("Показывать все сразу")
                    .on_toggle(WritingMessage::SwitchShowMode),
                text!("{}", self.roman_total).size(34),
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
        .center_y(Fill)
        .padding(10)
        .into()
    }

    fn answers(&self) -> Element<'_, WritingMessage> {
        if self.set.is_empty()  {
            text!("{}", self.kana_total).size(36).into()
        } else {
            space().height(36).into()
        }
    }
}
#[derive(Debug, Clone)]
pub enum WritingMessage {
    Next,
    Back,
    SwitchShowMode(bool),
}
