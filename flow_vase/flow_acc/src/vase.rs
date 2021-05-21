use flow_vessel::{Entity, TimeUnique};
use yew::{html, Component, ComponentLink};

use crate::components::block::EntityBlock;

pub struct Vase {
    entity: Entity,
    link: ComponentLink<Self>,
}

pub enum Msg {
    Entity(Entity),
}

impl Component for Vase {
    type Message = Msg;

    type Properties = ();

    fn create(
        _props: Self::Properties,
        link: yew::ComponentLink<Self>,
    ) -> Self {
        let id = TimeUnique::default();
        let entity = Entity::new_id(&id);
        Self { entity, link }
    }

    fn update(&mut self, msg: Self::Message) -> yew::ShouldRender {
        match msg {
            Msg::Entity(entity) => self.entity = entity,
        }
        true
    }

    fn change(&mut self, _props: Self::Properties) -> yew::ShouldRender {
        true
    }

    fn view(&self) -> yew::Html {
        html! {
            <EntityBlock entity=self.entity.clone() entity_update=self.link.callback(move |entity| Msg::Entity(entity))/>
        }
    }
}
