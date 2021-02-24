// use crate::util::*;
// use crate::yew_util::*;

use crate::universe::*;
use super::prelude::*;

pub struct LinearCube {
    props: Props,
}

#[derive(Clone, Properties)]
pub struct Props {
    entity_map: HashMap<EntityId, Entity>,
    linear: Linear<EntityId>,
}

impl Component for LinearCube {
    type Message = ();
    type Properties = Props;
    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self { 
        Self { props }
    }
    fn update(&mut self, _msg: Self::Message) -> ShouldRender { 
        true
    }
    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props = props; 
        true
    }
    fn view(&self) -> Html { 
        html! {
            <section class="linear">
                <h1>{self.props.linear.title.as_str()}</h1>
                <div class="node-group">

                </div>
            </section>
        }
    }
}
