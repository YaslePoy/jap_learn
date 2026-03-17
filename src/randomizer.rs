
pub mod randomizer {
    use crate::randomizer::randomizer::RandomizerMessage::{Back, Start};
    use crate::{NavigatedPage, Page, RootMessage};
    use iced::widget::{button, container, text_editor};
    use iced::Task;
    use rand::prelude::SliceRandom;

    #[derive(Clone, Debug)]
    pub struct RandomizerState {
        text: text_editor::Content,
        list: Vec<String>
    }

    #[derive(Debug, Clone)]
    pub enum RandomizerMessage {
        Back,
        Start,
        Edit(text_editor::Action),
    }

    impl NavigatedPage<RandomizerMessage> for RandomizerState {
        fn navigate(&self, message: &RandomizerMessage) -> Option<Page> {
            if let RandomizerMessage::Back = message {
                return Some(Page::PreviousPage);
            }
            None
        }
    }

    impl Default for RandomizerState {
        fn default() -> Self {
            Self::new()
        }
    }

    impl RandomizerState {
        pub fn new() -> RandomizerState {
            RandomizerState { text: Default::default(), list: vec![] }
        }

        pub fn update(&mut self, message: RandomizerMessage) -> Task<RootMessage> {
            match message {
                RandomizerMessage::Edit(action) => {
                    self.text.perform(action);
                    self.list = self.text.text().split("\n").map(|s| s.to_string()).collect();
                },
                RandomizerMessage::Start => {
                    self.list.shuffle(&mut rand::rng());
                    self.text = text_editor::Content::with_text(self.list.join("\n").as_str());
                }
                _ => {}
            }
            Task::none()
        }

        pub fn view(&self) -> iced::Element<'_, RandomizerMessage> {
            container(
                    iced::widget::column![
                        button("Назад").on_press(Back),
                        text_editor(&self.text).on_action(RandomizerMessage::Edit
                        ).width(400).height(400).placeholder("Каждый элемент с новой строки"),
                        button("Начать").on_press(Start),
                    ]
                    .spacing(5),
            )
            .padding(10)
            .into()
        }
    }
}
