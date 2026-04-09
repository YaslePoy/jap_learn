use crate::Page::PreviousPage;
use crate::{NavigatedPage, Page, RootMessage};
use iced::widget::{button, container};
use iced::{Element, Fill, Left, Task};

#[derive(Clone, Debug)]
pub struct RepetitionState {
}

impl Default for RepetitionState {
    fn default() -> Self {
        RepetitionState::new()
    }
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
    pub(crate) fn new() -> RepetitionState {
        RepetitionState {

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
