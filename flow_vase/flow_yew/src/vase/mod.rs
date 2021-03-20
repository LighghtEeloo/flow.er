mod view;

use yew::{Component, ComponentLink, Html, ShouldRender};
use flow_vessel::{Router, Vessel};

pub struct Vase {
    vessel: Vessel,
    link: ComponentLink<Self>,
}

pub enum Msg {
    SwitchRouter(Router)
}

impl Component for Vase {
    type Message = Vec<Msg>;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            vessel: Vessel::new(),
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
