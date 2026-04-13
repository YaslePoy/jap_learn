use std::sync::{Arc, Mutex};
use crate::Page::PreviousPage;
use crate::{AppState, NavigatedPage, Page, RootMessage};
use iced::widget::{button, container};
use iced::{Element, Fill, Left, Task};
use crate::lang::CardSet;
use crate::repetitions::CardSetSettings;

#[derive(Clone)]
pub struct RepetitionState {
    pub set: CardSetSettings,
    pub state: Arc<Mutex<AppState>>,
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
        RepetitionState {
            set,
            state
        }
    }
}

impl RepetitionState {
    pub fn update(&mut self, message: RepetitionMessage) -> Task<RootMessage>  {
        Task::none()
    }


    pub fn view(&self) -> Element<'_, RepetitionMessage> {
        container(
            iced::widget::column![
                button("Назад").on_press(RepetitionMessage::Back),

            ].align_x(Left).width(Fill)
        )
            .center_x(Fill)
            .padding(10)
            .into()
    }
}
#[derive(Debug, Clone)]
pub enum RepetitionMessage {
    Next,
    Back,
    SwitchShowMode(bool),
}
