use crate::util::*;
use crate::yew_util::*;
use super::prelude::*;


/// Linear
/// A Visualizable model for CubeView. Designed for todo-list-ish situations.
#[derive(Clone, Deserialize, Serialize)]
pub struct Linear<Id>
where Id: Identity
{
    pub title: Id,
    pub vec: Vec<Id>,
    #[serde(skip)]
    refs: HashMap<Id, NodeRef>,
    pub pos: Option<Id>,
    pub fix: FixState<Id>,
    pub locked: bool,
}

impl<Id> Linear<Id>
where Id: Identity 
{
    pub fn from_flow(flow: &Flow<Id>, target: &Id) -> Self {
        let vec = flow.get(target, "linear build failed").descendant.clone();
        let mut refs = HashMap::from_iter(vec.iter().cloned().map(|x| (x, NodeRef::default())) );
        refs.insert(target.clone(), NodeRef::default());
        Self {
            title: target.clone(),
            vec,
            refs,
            pos: None,
            fix: FixState::Deactivated,
            locked: false
        }
    }
    pub fn from_flow_boxed(flow: &Flow<Id>, target: &Id) -> Box<Self> {
        Box::new(Self::from_flow(flow, target))
    }
}


// Artist

impl Artist<EntityId> for Linear<EntityId> {}


// Animator

impl Animator<EntityId> for Linear<EntityId> {
    fn flow_update(&mut self, flow: &Flow<EntityId>) {
        self.vec = flow.get(&self.title, "linear update failed").descendant.clone();
        self.refs = HashMap::from_iter(self.vec.iter().cloned().map(|x| (x, NodeRef::default())) );
        self.refs.insert(self.title.clone(), NodeRef::default());
    }
    fn illustrate(&self, vm_idx: usize, vessel: &Vessel, link: &ComponentLink<Vase>) -> Html {
        let title_entity = vessel.entity_map.get(&self.title).cloned().unwrap_or_default();
        let vec_entity: Vec<Entity> = self.vec.iter().map(|id| vessel.entity_map.get(id).cloned().unwrap_or_default()).collect();
        html! {
            <div class="linear">
                <div class="head">
                    <input
                        type="Text"
                        ref=self.refs.get(&self.title).cloned().unwrap_or_default()
                        placeholder="Enter node name."
                        aria-label="New Project Name"
                        value=title_entity.face
                        // Todo..
                        oninput=link.callback(move |e: InputData| {
                            [VaseMsg::WriteEntity(title_entity.id(), EntityField::Face(e.value))]
                        })
                    />
                </div>
                <div class="node-group">
                    { for vec_entity.iter().map(|entity| { self.node_view(entity, link) }) }
                </div>
            </div>
        }
    }
}
impl Linear<EntityId> {
    fn node_view(&self, entity: &Entity, link: &ComponentLink<Vase>) -> Html {
        html! {
            <div class="node">
                { self.node_status_view(&entity, link) }
                { self.node_input_view(&entity, link) }
            </div>
        }
    }
    fn node_status_view(&self, entity: &Entity, link: &ComponentLink<Vase>) -> Html {
        let id = entity.id();
        let vec = ProcessStatus::vec_all();
        let status_meta: Vec<(String, String, ProcessStatus)> = 
            vec.iter().map( |x| (
                String::from(ProcessStatus::type_src(x)), 
                String::from(ProcessStatus::type_str(x)),
                x.clone()
            ) ).collect();
        let status_dropdown: Html = 
            status_meta.into_iter().map(|(src, des, process)| {
                html! {
                    <div title={des.clone()}
                        onclick=link.callback(move |_| {
                            [VaseMsg::WriteEntity(id, EntityField::ProcessStatus(process.clone()))]
                        })
                    > 
                        <img src={src} alt="process" /> 
                    </div> 
                }
            }).collect();
        html! {
            <div class="dropdown"> 
                <button class="dropbtn"
                    value=entity.process.type_str()
                > 
                    <img src={entity.process.type_src()} alt="process" />
                </button> 
                
                <div class="dropdown-content"> 
                    { status_dropdown }
                </div> 
            </div> 
        }
    }
    fn node_input_view(&self, entity: &Entity, link: &ComponentLink<Vase>) -> Html {
        let mut entity = entity.clone();
        let id = entity.id();
        let is_empty = entity.face.is_empty();
        html! {
            <input
                type="text"
                ref=self.refs.get(&id).unwrap().clone()
                value=entity.face
                placeholder="..."
                aria-label="Item"
                // onfocus=link.callback(move |_| {
                //     Cubey![SetFocusId(Some(id))]
                // })
                // onkeydown=link.callback(move |e: KeyboardEvent| {
                //     let meta = (e.ctrl_key(), e.shift_key(), e.code());
                //     // LOG!("OnKeyDown: {:?}", meta);
                //     match (meta.0, meta.1, meta.2.as_str()) { 
                //         (false, false, "ArrowUp") => Cubey![Wander(Direction::Ascend, false)], 
                //         (false, false, "ArrowDown") => Cubey![Wander(Direction::Descend, false)], 
                //         (true, false, "ArrowUp") => Cubey![Wander(Direction::Ascend, true)], 
                //         (true, false, "ArrowDown") => Cubey![Wander(Direction::Descend, true)], 
                //         (false, false, "ArrowLeft") => Cubey![], 
                //         (false, false, "ArrowRight") => Cubey![], 
                //         _ => Cubey![]
                //     }
                // })
                // onkeypress=link.callback(move |e: KeyboardEvent| {
                //     let meta = (e.ctrl_key(), e.shift_key(), e.code());
                //     // LOG!("OnKeyPress: {:?}", meta);
                //     match (meta.0, meta.1, meta.2.as_str()) { 
                //         _ => Cubey![]
                //     }
                // })
                // onkeyup=link.callback(move |e: KeyboardEvent| {
                //     let meta = (e.ctrl_key(), e.shift_key(), e.code());
                //     // LOG!("OnKeyUp: {:?}", meta);
                //     match (meta.0, meta.1, meta.2.as_str()) { 
                //         // enter
                //         (false, false, "Enter") => Cubey![NewNode(vec![id])],
                //         // shift+enter
                //         (false, true, "Enter") => Cubey![],
                //         // backspace
                //         (_, _, "Backspace") => {
                //             if is_empty { Cubey![EraseNode(id)] }
                //             else { Cubey![] }
                //         }
                //         // delete
                //         (_, _, "Delete") => {
                //             if is_empty { Cubey![EraseNode(id), EraseNode(id), Wander(Direction::Descend, false)] }
                //             else { Cubey![] }
                //         }
                //         // ctrl released
                //         (true, _, "ControlLeft") => Cubey![Wander(Direction::Stay, false)],
                //         (true, _, "ControlRight") => Cubey![Wander(Direction::Stay, false)],
                //         _ => Cubey![] 
                //     }
                // })
                oninput=link.callback(move |e: InputData| {
                    [VaseMsg::WriteEntity(id, EntityField::Face(e.value))]
                })
                readonly=self.locked
            />
        }
    }
}


// Dancer

impl<Id> Dancer<Id> for Linear<Id>
where Id: Identity
{
    fn check(&self, obj: &Id) -> Result<Id, Critic> {
        if self.vec.contains(obj) { Ok(*obj) } else { Err(FlowNodeNotFoundError) }
    }
    fn current(&self) -> Option<Id> {
        self.pos.clone()
    }
    fn focus(&mut self, obj: Id) {
        // validate obj.
        self.check(&obj).expect("trying to focus none");
        self.pos = Some(obj)
    }
    fn wander(&mut self, dir: Direction, fixed: bool) {
        if self.vec.is_empty() { return }
        if Direction::Stay == dir && fixed == false {
            self.fix.deactivate();
            return
        }
        // Todo: migrate linear in stockpile.
        if fixed {

        } else {

        }
    }
}


impl<Id> Debug for Linear<Id> 
where Id: Identity
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        f.debug_struct("Linear")
         .field("title", &self.title)
         .field("vec", &self.vec)
         .field("pos", &self.pos)
         .field("fix", &self.fix)
         .field("locked", &self.locked)
         .finish()
    }
}
