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
    fn illustrate(&self, vm_meta: VMMeta, vessel: &Vessel, link: &ComponentLink<Vase>) -> Html {
        let (vm_router, vm_idx) = vm_meta;
        let title_entity = vessel.entity_map.get(&self.title).cloned().unwrap_or_default();
        let title_id = title_entity.id();
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
                        onfocus=link.callback(move |_| {
                            Vasey![SetFocusId(vm_meta, title_id)]
                        })
                        onkeydown=link.callback(move |e: KeyboardEvent| {
                            let meta = (e.ctrl_key(), e.shift_key(), e.code());
                            LOG!("OnKeyDown: {:?}", meta);
                            match (meta.0, meta.1, meta.2.as_str()) { 
                                (false, false, "ArrowDown") => Vasey![Wander(vm_meta, Direction::Descend, false)], 
                                _ => Vasey![]
                            }
                        })
                        // onkeyup=link.callback(move |e: KeyboardEvent| {
                        //     LOG!("OnKeyUp: {:?}", e);
                        //     if e.key() == "Enter" { Vasey![AddNode] } else { Vasey![] }
                        // })
                        oninput=link.callback(move |e: InputData| {
                            Vasey![WriteEntity(title_id, EntityField::Face(e.value))]
                        })
                    />
                </div>
                <div class="node-group">
                    { for vec_entity.iter().map(|entity| { self.node_view(vm_meta, entity, link) }) }
                </div>
            </div>
        }
    }
}
impl Linear<EntityId> {
    fn node_view(&self, vm_meta: VMMeta, entity: &Entity, link: &ComponentLink<Vase>) -> Html {
        html! {
            <div class="node">
                { self.node_status_view(&entity, link) }
                { self.node_input_view(vm_meta, &entity, link) }
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
                            Vasey![WriteEntity(id, EntityField::ProcessStatus(process.clone()))]
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
    fn node_input_view(&self, vm_meta: VMMeta, entity: &Entity, link: &ComponentLink<Vase>) -> Html {
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
                onfocus=link.callback(move |_| {
                    Vasey![SetFocusId(vm_meta, id)]
                })
                onkeydown=link.callback(move |e: KeyboardEvent| {
                    let meta = (e.ctrl_key(), e.shift_key(), e.code());
                    // LOG!("OnKeyDown: {:?}", meta);
                    match (meta.0, meta.1, meta.2.as_str()) { 
                        (false, false, "ArrowUp") => Vasey![Wander(vm_meta, Direction::Ascend, false)], 
                        (false, false, "ArrowDown") => Vasey![Wander(vm_meta, Direction::Descend, false)], 
                        (true, false, "ArrowUp") => Vasey![Wander(vm_meta, Direction::Ascend, true)], 
                        (true, false, "ArrowDown") => Vasey![Wander(vm_meta, Direction::Descend, true)], 
                        (false, false, "ArrowLeft") => Vasey![], 
                        (false, false, "ArrowRight") => Vasey![], 
                        _ => Vasey![]
                    }
                })
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
                //         (false, false, "Enter") => Cubey![AddNode()],
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
                    Vasey![WriteEntity(id, EntityField::Face(e.value))]
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
        if self.vec.contains(obj) || *obj == self.title { Ok(*obj) } else { Err(FlowNodeNotFoundError) }
    }
    fn current(&self) -> Option<Id> {
        self.pos.clone()
    }
    fn current_ref(&self) -> Option<NodeRef> {
        self.current().map(|id| {self.refs.get(&id).cloned()}).flatten()
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
        // Note: a pair of insert and remove!
        self.vec.insert(0, self.title);
        LOG!("{:?}, {:?}", dir, fixed);
        LOG!("{:?}", self.vec);
        let try_move = |delta: isize| -> Option<usize> {
            let current = self.current()?;
            let pos = self.vec.iter().position(|&x| x == current)? as isize;
            let pos = pos + delta;
            let pos = if pos < 0 { 0 } else if pos < self.vec.len() as isize { pos as usize } else { self.vec.len() - 1 };
            Some(pos)
        };
        if fixed {

        } else {
            try_move(dir.translate()).map(|pos| {
                self.focus(self.vec[pos])
            });
        }
        self.vec.remove(0);
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
