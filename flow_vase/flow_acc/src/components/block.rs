use flow_vessel::{Entity, EntityField};
use yew::{
    html, Callback, Component, ComponentLink, InputData, KeyboardEvent,
    Properties,
};

pub struct EntityBlock {
    props: Props,
    link: ComponentLink<Self>,
}

#[derive(Clone, Properties, PartialEq)]
pub struct Props {
    pub entity: Entity,
    pub entity_update: Callback<Entity>,
}

pub enum Msg {
    KeyDown(KeyboardEvent),
    KeyUp(KeyboardEvent),
    Input(String),
}

impl Component for EntityBlock {
    type Message = Msg;

    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { props, link }
    }

    fn update(&mut self, msg: Self::Message) -> yew::ShouldRender {
        match msg {
            Msg::KeyDown(e) => match e.code().as_str() {
                "ArrowUp" | "ArrowDown" => e.prevent_default(),
                _ => {}
            },
            Msg::KeyUp(e) => e.prevent_default(),
            Msg::Input(val) => {
                let mut entity = self.props.entity.clone();
                entity.update_entity(EntityField::Face(val));
                self.props.entity_update.emit(entity);
            }
        }
        true
    }

    fn change(&mut self, props: Self::Properties) -> yew::ShouldRender {
        if self.props != props {
            self.props = props;
            log::debug!("\nentity updated in block: {:#?}", self.props.entity);
            true
        } else {
            false
        }
    }

    fn view(&self) -> yew::Html {
        html! {
            <input
                onkeydown=self.link.callback(move |e| Msg::KeyDown(e))
                onkeyup=self.link.callback(move |e| Msg::KeyUp(e))
                oninput=self.link.callback(move |e: InputData| Msg::Input(e.value))
            />
        }
    }
}
