use std::default;

use iced::{Column, Element, HorizontalAlignment, Length, Text, TextInput, button, scrollable, text_input};

pub fn main_gui() -> iced::Result {
    todo!()
}

#[derive(Debug, Default)]
struct FLowerState {
    left_router: Vec<button::State>,
    main_editor: scrollable::State,
    title: text_input::State,
    new_item: text_input::State,
    list: Vec<text_input::State>,
    vessel: flow_vessel::Vessel
}

type Message = ();

impl iced::Application for FLowerState {
    type Executor = iced::executor::Default;
    type Message = Message;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, iced::Command<Self::Message>) {
        (
            FLowerState::default(),
            // iced::Command::perform(SavedState::load(), Message::Loaded),
            iced::Command::none()
        )
    }

    fn title(&self) -> String {
        "".into()
    }

    fn update(&mut self, message: Self::Message) -> iced::Command<Self::Message> {
        match message {
            _ => iced::Command::none()
        }
    }

    fn view(&mut self) -> iced::Element<'_, Self::Message> {
        let Self {
            left_router,
            vessel,
            ..
        } = self;
        let title = Text::new("todos")
            .width(Length::Fill)
            .size(100)
            .color([0.5, 0.5, 0.5])
            .horizontal_alignment(HorizontalAlignment::Center);

        let input = TextInput::new(
            input,
            "What needs to be done?",
            input_value,
            || ()
        )
        .padding(15)
        .size(30)
        .on_submit(());

        let tasks: Element<_> = 
            tasks
                .iter_mut()
                .enumerate()
                .filter(|(_, task)| filter.matches(task))
                .fold(Column::new().spacing(20), |column, (i, task)| {
                    column.push(task.view().map(move |message| {
                        || ()
                    }))
                })
                .into();

        let content = Column::new()
            .max_width(800)
            .spacing(20)
            .push(title)
            .push(input)
            .push(controls)
            .push(tasks);

        Scrollable::new(scroll)
            .padding(40)
            .push(
                Container::new(content).width(Length::Fill).center_x(),
            )
            .into()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
