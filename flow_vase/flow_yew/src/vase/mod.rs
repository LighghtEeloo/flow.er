mod main;

use yew::{html, Component, ComponentLink, Html, ShouldRender};

use main::main_view;

pub struct Vase {
    link: ComponentLink<Self>
}

pub enum Message {

}

impl Component for Vase {
    type Message = Message;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        todo!()
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {
        self.main_view()
    }
}
