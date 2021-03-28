use std::collections::HashMap;

use yew::{ComponentLink, Html, InputData, NodeRef, html};
use flow_vessel::{Cube, CubeMeta, EntityField, EntityId, Vessel};
use super::{CubeView, Vase, Msg::*, btn};

#[derive(Clone)]
pub struct FlowView {
    obj: EntityId,
    link: ComponentLink<Vase>,
    node_ref_map: HashMap<EntityId, NodeRef>
}

impl Into<CubeView> for FlowView {
    fn into(self) -> CubeView {
        CubeView::FlowView {
            flow_view: self
        }
    }
}

impl FlowView {
    pub fn new_cube(cube: Cube, vessel: &Vessel, link: ComponentLink<Vase>) -> CubeView {
        let obj = cube.obj.unwrap_or_default();
        // Todo: get all entity_ids recrusively.
        let entity_ids = vessel.entity_ids(&obj);
        let flow_view = 
            Self {
                obj,
                link,
                node_ref_map: HashMap::from(
                    entity_ids.into_iter().map(|x| (x, NodeRef::default())).collect()
                )
            };
        flow_view.into()
    }
    pub fn update_new(mut self, cube: &Cube, vessel: &Vessel) -> CubeView {
        let blank = CubeView::default();
        if let Some(obj) = cube.obj {
            if vessel.entity_get(&obj).is_some() {
                self.obj = obj;
                self.into()
            } else {
                blank
            }
        } else {
            blank
        }
    }
    pub fn view(&self, vessel: &Vessel, meta: &CubeMeta) -> Html {
        node(&self.obj, 0, vessel, "--".into(), self.link.clone())
    }
}

fn expand(obj: &EntityId, vessel: &Vessel) -> Vec<EntityId> {
    vessel.node(obj).unwrap().children.clone()
}

fn node(id: &EntityId, idx: usize, vessel: &Vessel, prefix: String, link: ComponentLink<Vase>) -> Html {
    let mut next_prefix = prefix.clone();
    next_prefix.push_str("--");
    let entity = vessel.entity_get(id).cloned().unwrap_or_default();
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
            <span>{prefix}{entity.face}</span>
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
