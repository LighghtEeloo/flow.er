use crate::util::*;
use crate::yew_util::*;
use super::prelude::*;


/// Inkblot
/// Shows bubble of the entity.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Inkblot<Id>
where Id: Identity
{
    pub target: Id,
    // #[serde(skip)]
    // refs: HashMap<Id, NodeRef>,
    pub pos: Option<Id>,
}

impl<Id> Inkblot<Id>
where Id: Identity 
{
    pub fn from_flow(flow: &Flow<Id>, target: &Id) -> Self {
        Self {
            target: *target,
            pos: None,
        }
    }
    pub fn from_flow_boxed(flow: &Flow<Id>, target: &Id) -> Box<Self> {
        Box::new(Self::from_flow(flow, target))
    }
}


// Artist

impl Artist<EntityId> for Inkblot<EntityId>  {}


// Animator

impl Animator<EntityId> for Inkblot<EntityId>  {
    fn flow_update(&mut self, flow: &Flow<EntityId>) {}
    fn illustrate(&self, vm_meta: VMMeta, vessel: &Vessel, link: &ComponentLink<Vase>) -> Html {
        let entity = vessel.entity_map.get(&self.target).cloned().expect("Host invalid.");
        let owner_id = entity.id();
        html! {
            <div class="inkblot">
                <div class="head">
                    <input
                            type="Text"
                            placeholder="An arbitrary node."
                            aria-label="Arbitrary Node"
                            value=entity.face
                            onfocus=link.callback(move |_| {
                                Vasey![SetFocusId(vm_meta, owner_id)]
                            })
                            onkeydown=link.callback(move |e: KeyboardEvent| {
                                let meta = (e.ctrl_key(), e.shift_key(), e.code());
                                match (meta.0, meta.1, meta.2.as_str()) { 
                                    (false, false, "ArrowDown") => Vasey![Wander(vm_meta, Direction::Descend, false)], 
                                    _ => Vasey![]
                                }
                            })
                            onkeyup=link.callback(move |e: KeyboardEvent| {
                                match e.code().as_str() { 
                                    "Enter" => Vasey!
                                        [ AddEntity(FlowLink::new_descend_head(owner_id))
                                        , Wander(vm_meta, Direction::Descend, false)
                                        ],
                                    _ => Vasey![] 
                                }
                            })
                            oninput=link.callback(move |e: InputData| {
                                Vasey![WriteEntity(owner_id, EntityField::Face(e.value))]
                            })
                        />
                    </div>
                <textarea class="bubble"
                    value=entity.bubble
                    type="text" 
                    oninput=link.callback(move |e: InputData| {
                        Vasey![WriteEntity(entity.id(), EntityField::Bubble(e.value))]
                    })
                    spellcheck=false
                />
            </div>
        }
    }
}


// Dancer

impl<Id> Dancer<Id> for Inkblot<Id>
where Id: Identity
{
    fn check(&self, obj: &Id) -> Result<Id, Critic> {
        Ok(*obj)
    }
    fn current(&self) -> Option<Id> {
        self.pos.clone()
    }
    
    fn current_ref(&self) -> Option<NodeRef> {
        None
    }
    fn focus(&mut self, obj: Id) {
    }
    fn wander(&mut self, dir: Direction, fixed: bool) {
    }
}

