mod view;

use yew::{Component, ComponentLink, Html, ShouldRender};
use flow_vessel::{Router, Vessel};

pub struct Vase {
    vessel: Vessel,
    link: ComponentLink<Self>,
}

#[derive(Debug, Clone)]
pub enum Msg {
    SwitchRouter(Router),
    Refresh
}

impl Component for Vase {
    // a conditional queue.
    type Message = Vec<Msg>;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            vessel: Vessel::new(),
            link
        }
    }

    fn update(&mut self, msg_queue: Self::Message) -> ShouldRender {
        let mut msg_visitor = msg_queue.into_iter();
        while {
            let next = msg_visitor.next();
            if let Some(msg) = next {
                use Msg::*;
                // returns true if non-block, false if needs revisit or finish
                match msg {
                    SwitchRouter(router) => {
                        self.vessel.router = router;
                        true
                    },
                    Refresh => false,
                }
            } else {
                // quit loop
                false
            }
        } {}
        let left: Vec<Msg> = msg_visitor.collect();
        if !left.is_empty() {
            self.link.callback(move |()| left.clone()).emit(());
            false
        } else {
            true
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {
        self.main_view()
    }
}
