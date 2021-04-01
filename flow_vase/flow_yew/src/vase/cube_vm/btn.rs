use yew::{ComponentLink, Html, html};
use flow_vessel::{CubeMeta, EntityField, EntityId};

use super::{Vase, Msg::*};

pub fn ink(meta: CubeMeta, obj: EntityId, style: String, link: ComponentLink<Vase>) -> Html {
    html! {
        <button class="btn-ink btn-operate" style=style
            onclick=link.callback(move |_| {
                [OpenVM{
                    cube: flow_vessel::cubes::Inkblot { obj }.into(),
                    meta
                }]
            })
        >{">"}</button>
    }
}

pub fn add(dude: EntityId, owner: EntityId, idx: usize, style: String, link: ComponentLink<Vase>) -> Html {
    html! {
        <button class="btn-add btn-operate" style=style
            onclick=link.callback(move |_| {
                [EntityAdd{
                    dude,
                    owner,
                    idx
                }]
            })
        >{"+"}</button>
    }
}

pub fn del(id: EntityId, style: String, link: ComponentLink<Vase>) -> Html {
    html! {
        <button class="btn-del btn-operate" style=style
            onclick=link.callback(move |_| {
                [EntityDelete{id}]
            })
        >{"✕"}</button>
    }
}

pub fn dive(id: EntityId, idx: usize, style: String, link: ComponentLink<Vase>) -> Html {
    html! {
        <button class="btn-add btn-operate" style=style
            onclick=link.callback(move |_| {
                [EntityDive { id, idx }]
            })
        >{"]"}</button>
    }
}

pub fn emerge(id: EntityId, style: String, link: ComponentLink<Vase>) -> Html {
    html! {
        <button class="btn-add btn-operate" style=style
            onclick=link.callback(move |_| {
                [EntityEmerge { id }]
            })
        >{"["}</button>
    }
}

pub fn up(id: EntityId, style: String, link: ComponentLink<Vase>) -> Html {
    html! {
        <button class="btn-up btn-operate" style=style
            onclick=link.callback(move |_| {
                [EntityUp { id }]
            })
        >{"u"}</button>
    }
}

pub fn down(id: EntityId, style: String, link: ComponentLink<Vase>) -> Html {
    html! {
        <button class="btn-down btn-operate" style=style
            onclick=link.callback(move |_| {
                [EntityDown { id }]
            })
        >{"d"}</button>
    }
}

pub fn block(id: EntityId, style: String, link: ComponentLink<Vase>) -> Html {
    html! {
        <button class="btn-block btn-operate" style=style
            onclick=link.callback(move |_| {
                [EntityUpdate {
                    id,
                    field: EntityField::Blocked 
                }]
            })
        >{"o"}</button>
    }
}
