use yew::{Component, ComponentLink, NodeRef, Properties};
use std::collections::HashMap;
use flow_vessel::{ClauseTreeCore, EntityId, Tube};
use super::Vase;



pub struct ClauseTree {
    props: Props,
    ref_map: HashMap<EntityId, NodeRef>,
    link: ComponentLink<Self>
}

#[derive(Properties, Clone)]
pub struct Props {
    core: ClauseTreeCore,
    link_tube: ComponentLink<Vase>
}

pub enum Msg {
    Tube (Tube)
}

impl Component for ClauseTree {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: yew::ComponentLink<Self>) -> Self {
        let ref_map = props.core.node_map.keys()
            .map(|x| {
                (x.clone(), NodeRef::default())
            }).collect();
        Self {
            props,
            ref_map,
            link
        }
    }

    fn update(&mut self, msg: Self::Message) -> yew::ShouldRender {
        todo!()
    }

    fn change(&mut self, props: Self::Properties) -> yew::ShouldRender {
        self.props = props;
        true
    }

    fn view(&self) -> yew::Html {
        todo!()
    }
}
