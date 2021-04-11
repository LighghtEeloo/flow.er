use std::collections::HashMap;

use yew::{ComponentLink, Html, NodeRef, html};
use flow_vessel::{CubeMeta, EntityId, Vessel, cubes::FlowView};
use super::{CubeView, Vase, btn};


impl CubeView for FlowView {
    fn view(&self, vessel: &Vessel, _meta: CubeMeta, link: ComponentLink<Vase>, _ref_map: &HashMap<EntityId, NodeRef>) -> Html {
        node(&self.obj, 0, vessel, "--".into(), link.clone())
    }
}

fn expand(obj: &EntityId, vessel: &Vessel) -> Vec<EntityId> {
    vessel.node(obj).unwrap().children.clone()
}

fn node(id: &EntityId, idx: usize, vessel: &Vessel, prefix: String, link: ComponentLink<Vase>) -> Html {
    let mut next_prefix = prefix.clone();
    next_prefix.push_str("--");
    let entity = vessel.entity(id).cloned().unwrap_or_default();
    let owner = vessel.node(id).unwrap().parent.unwrap_or_default();
    let dude = owner;
    let children: Vec<Html> = expand(id, vessel)
        .iter().enumerate()
        .map(|(i, x)| 
            node(x, i, vessel, next_prefix.clone(), link.clone())
        ).collect();
    let id = entity.id().clone();
    html! {
        <> 
            <span style="font-family: mono">{prefix}{entity.face}</span>
            {btn::up(id, "".into(), link.clone())}
            {btn::down(id, "".into(), link.clone())}
            {btn::emerge(id, "".into(), link.clone())}
            {btn::dive(id, idx, "".into(), link.clone())}
            {btn::add(dude, owner, idx + 1, "".into(), link.clone())}
            {btn::del(id, "".into(), link.clone())}
            <div>
                {children}
            </div>
        </>
    }
}
