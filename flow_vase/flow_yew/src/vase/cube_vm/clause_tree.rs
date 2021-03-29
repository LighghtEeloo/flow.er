use yew::{ComponentLink, Html, InputData, KeyboardEvent, NodeRef, html};
use flow_vessel::{Cube, CubeMeta, Entity, EntityField, EntityId, EntityNode, Lint, Process, Symbol, Vessel};
use super::{Vase, Msg::*, CubeView, btn};

#[derive(Clone)]
pub struct ClauseNode {
    id: EntityId,
    link: ComponentLink<Vase>,
    node_ref: NodeRef,
}


impl ClauseNode {
    pub fn new_cube(id: EntityId, link: ComponentLink<Vase>) -> Self {
        Self {
            id,
            link,
            node_ref: NodeRef::default(),
        }
    }
    pub fn view(&self, idx: usize, entity: &Entity, owner: EntityId, meta: &CubeMeta) -> Html {
        let id = entity.id().clone();
        html! {
            <div class="node">
                { self.symbol_view(idx, &entity) }
                { self.input_view(idx, &entity, owner) }
                { btn_ink(meta.incr_new(), id, self.link.clone()) }
                // { btn_add(id, owner, idx + 1, self.link.clone()) }
                { btn_del(id, self.link.clone()) }
            </div>
        }
    }
    fn symbol_view(&self, idx: usize, entity: &Entity) -> Html {
        let indent_base = 0;
        let indent = indent_base;
        let id = entity.id().clone();
        let symbol = match (entity.symbol_toggle, entity.symbol.clone()) {
            (false, Symbol::ProcessTracker(process)) => 
                self.process(id, process),
            (false, Symbol::Linted(lint)) =>
                self.lint(id, idx, lint),
            _ => {
                // list the toggle options
                let contents: Html = html! {
                    <>
                        <div title="toggle-linted"
                            onclick=self.link.callback(move |_| {
                                [ EntityUpdate {
                                    id, 
                                    // id: id.clone(), 
                                    field: EntityField::Symbol(Symbol::Linted(Lint::default()))
                                } ]
                            })
                        >
                            <span>{Lint::default().display(0)}</span>
                        </div>
                        <div title="toggle-process-tracker"
                            onclick=self.link.callback(move |_| {
                                [ EntityUpdate {
                                    id, 
                                    // id: id.clone(), 
                                    field: EntityField::Symbol(Symbol::ProcessTracker(Process::default()))
                                } ]
                            })
                        >
                            <img src={Process::type_src(&Process::default())} alt="process" /> 
                        </div>
                    </>
                };
                html! {
                    <div class="dropdown-content" style="display: block">
                        { contents }
                    </div>
                }
            }
        };
        let style = 
            format!("left: calc({} * var(--size-button) + {}px);", indent, indent);
        html! {
            <div class="symbol" style=style> 
                { symbol }
            </div> 
        }
    }
    fn input_view(&self, idx: usize, entity: &Entity, owner: EntityId) -> Html {
        let indent_base = 0;
        let indent = indent_base + 1;
        let id = entity.id().clone();
        let style = 
            format!("width: calc(100% - {} * var(--size-button) - var(--horizontal-margin) * 2);", indent);
        html! {
            <input
                type="text"
                ref=self.node_ref.clone()
                value=entity.face
                style=style
                placeholder="..."
                aria-label="Item"
                // onfocus=self.link.callback(move |_| {
                //     vec![SetFocusId(vm_meta, id)]
                // })
                onkeydown=self.link.callback(move |e: KeyboardEvent| {
                    let meta = (e.ctrl_key(), e.shift_key(), e.code());
                    match (meta.0, meta.1, meta.2.as_str()) { 
                //         (false, false, "ArrowUp") => vec!
                //             [Wander(vm_meta, Direction::Ascend, false)], 
                //         (false, false, "ArrowDown") => vec!
                //             [Wander(vm_meta, Direction::Descend, false)], 
                //         (true, false, "ArrowUp") => vec!
                //             [Wander(vm_meta, Direction::Ascend, true)], 
                //         (true, false, "ArrowDown") => vec!
                //             [Wander(vm_meta, Direction::Descend, true)], 
                        (true, true, "ArrowUp") => vec![
                            EntityUp { id }
                        ], 
                        (true, true, "ArrowDown") => vec![
                            EntityDown { id }
                        ], 
                        (true, false, "BracketRight") => vec![
                            EntityDive { id, idx }
                        ], 
                        (true, false, "BracketLeft") => vec![
                            EntityEmerge { id }
                        ], 
                        _ => vec![]
                    }
                })
                onkeyup=self.link.callback(move |e: KeyboardEvent| {
                    let meta = (e.ctrl_key(), e.shift_key(), e.code());
                    match (meta.0, meta.1, meta.2.as_str()) { 
                        // enter
                        (false, false, "Enter") => vec!
                            [ EntityAdd { dude: id, owner, idx: idx+1 }
                            // , Wander(vm_meta, Direction::Descend, false)
                            ],
                //         // // shift+enter
                //         // (false, true, "Enter") => vec![],
                //         // backspace
                //         (_, _, "Backspace") => {
                //             if is_empty { vec!
                //                 [ EraseEntity(id)
                //                 , Wander(vm_meta, Direction::Descend, false)
                //                 ] 
                //             } else { vec![] }
                //         }
                //         // delete
                //         (_, _, "Delete") => {
                //             if is_empty { vec!
                //                 [ EraseEntity(id)
                //                 ] 
                //             } else { vec![] }
                //         }
                //         // // ctrl released
                //         // (true, _, "ControlLeft") => vec![Wander(Direction::Stay, false)],
                //         // (true, _, "ControlRight") => vec![Wander(Direction::Stay, false)],
                        _ => vec![] 
                    }
                })
                oninput=self.link.callback(move |e: InputData| {
                    [ EntityUpdate {
                        id, 
                        field: EntityField::Face(e.value)
                    } ]
                })
                // readonly=self.locked
            />
        }
    }
}

// symbol view
impl ClauseNode {
    fn process(&self, id: EntityId, process: Process) -> Html {
        let process_meta: Vec<(String, String, Process)> = 
        Process::vec_all().iter().map( |x| (
            String::from(Process::type_src(x)), 
            String::from(Process::type_str(x)),
            x.clone()
        ) ).collect();
        let dropdown: Html = 
            process_meta.into_iter().map(|(src, des, process)| {
                html! {
                    <div title={des.clone()}
                        onclick=self.link.callback(move |_| {
                            [ EntityUpdate {
                                id, 
                                // id: id.clone(), 
                                field: EntityField::Symbol(Symbol::ProcessTracker(process.clone()))
                            } ]
                        })
                    > 
                        <img src={src} alt="process" /> 
                    </div> 
                }
            }).collect();
        html! {    
            <>
                <button class="dropbtn process"
                    value=process.type_str()
                    onclick=self.link.callback(move|_| {
                        [EntityUpdate{
                            id,
                            field: EntityField::SymbolToggle
                        }]
                    })
                > 
                    <img src={process.type_src()} alt="process" />
                </button> 
                
                <div class="dropdown-content"> 
                    {dropdown}
                </div> 
            </>
        }
    }
    fn lint(&self, id: EntityId, idx: usize, lint: Lint) -> Html {
        let text = lint.display(idx);
        let lint_meta: Vec<(String, Lint)> = Lint::vec_all().into_iter().map(|x|
            (x.display(0), x)
        ).collect();
        let dropdown: Html = 
            lint_meta.into_iter().map(|(text, lint)| {
                html! {
                    <div title={lint.clone().type_str()}
                        onclick=self.link.callback(move |_| {
                            [ EntityUpdate {
                                id, 
                                // id: id.clone(), 
                                field: EntityField::Symbol(Symbol::Linted(lint.clone()))
                            } ]
                        })
                    > 
                        <span> {text} </span>
                    </div> 
                }
            }).collect();
        html! {
            <>
                <button class="dropbtn lint"
                    onclick=self.link.callback(move|_| {
                        [EntityUpdate{
                            id,
                            field: EntityField::SymbolToggle
                        }]
                    })
                > 
                    <div class="symbol-text"> {text} </div>
                </button> 

                <div class="dropdown-content"> 
                    {dropdown}
                </div> 
            </>
        }
    }
}


#[derive(Clone)]
pub struct ClauseTree {
    current: Option<usize>,
    head: ClauseNode,
    nodes: Vec<ClauseNode>
}

impl ClauseTree {
    pub fn new_cube(entity_node: &EntityNode, current: Option<usize>, link: ComponentLink<Vase>) -> CubeView {
        let mut nodes = Vec::new();
        let id = entity_node.entity.id();
        for id in entity_node.children.iter() {
            nodes.push(ClauseNode::new_cube(id.clone(), link.clone()))
        }
        let clause = Self {
            current,
            head: ClauseNode::new_cube(id.clone(), link),
            nodes
        };
        CubeView::ClauseTree {
            clause
        }
    }
    pub fn head_id(&self) -> EntityId {
        self.head.id
    }
    pub fn update_new(mut self, _cube: &Cube, vessel: &Vessel) -> CubeView {
        let entity_node =  vessel.node(&self.head_id());
        if let Some(entity_node) = entity_node {
            let link = self.head.link.clone();
            let correct = &entity_node.children;
            let _target = self.nodes.clone();
            // self.nodes = ClauseTree::_update_rebuild(correct, link);
            self.nodes = ClauseTree::_update_iter_impl(_target, correct, link);
            CubeView::ClauseTree { clause: self }
        } else {
            CubeView::default()
        }
    }
    fn _update_rebuild(correct: &Vec<EntityId>, link: ComponentLink<Vase>) -> Vec<ClauseNode> {
        correct.iter().map(|id| {
            ClauseNode::new_cube(id.clone(), link.clone())
        }).collect()
    }
    fn _update_iter_impl(mut target: Vec<ClauseNode>, correct: &Vec<EntityId>, link: ComponentLink<Vase>) -> Vec<ClauseNode> {
        // final effect: correct is identical to target
        for (i, c) in correct.iter().enumerate() {
            match target.get(i) {
                Some(node) => {
                    if &node.id != c {
                        let mut rest = target.iter().skip(i+1);
                        if rest.find(|&node| &node.id == c).is_some() {
                            while {
                                let res = target.get(i)
                                    .and_then(|node| {
                                        if &node.id != c {
                                            let mut node = node.clone();
                                            node.id = c.clone();
                                            Some(node)
                                        } else {
                                            None
                                        }
                                    }); 
                                res.is_some()
                            } {
                                target.remove(i);
                            }
                        }
                        if let Some(node) = target.get(i) {
                            if &node.id != c {
                                let mut node = node.clone();
                                node.id = c.clone();
                                target.insert(i, node);
                            }
                        }
                    } else {
                        ()
                    }
                },
                None => {
                    let node = ClauseNode::new_cube(c.clone(), link.clone());
                    target.insert(i, node);
                }
            }
        }
        target.truncate(correct.len());
        target
    }
    pub fn view(&self, vessel: &Vessel, meta: &CubeMeta) -> Html {
        let nodes_view: Vec<Html> = self.nodes.iter().enumerate()
            .map(|(idx, node)| {
                node.view(
                    idx, 
                    vessel.entity_get(&node.id).expect("must exist"), 
                    self.head.id,
                    meta
                )
            }).collect();
        html! {
            <>
                { self.head_view(vessel, meta) }
                <div class="node-view"> { nodes_view } </div>
            </>
        }
    }

    fn head_view(&self, vessel: &Vessel, meta: &CubeMeta) -> Html {
        let id = self.head_id();
        let entity = vessel.entity_get(&id).expect("Host doesn't exist.");
        let link = self.head.link.clone();
        html! {
            <div class="head">
                <input
                    type="Text"
                    placeholder="An arbitrary node."
                    aria-label="Arbitrary Node"
                    value=entity.face
                    onkeyup=link.callback(move |e: KeyboardEvent| {
                        let meta = (e.ctrl_key(), e.shift_key(), e.code());
                        match (meta.0, meta.1, meta.2.as_str()) { 
                            // enter
                            (false, false, "Enter") => vec!
                                [ EntityAdd { dude: id, owner: id, idx: 0 }
                                // , Wander(vm_meta, Direction::Descend, false)
                                ],
                    //         // // shift+enter
                    //         // (false, true, "Enter") => vec![],
                    //         // backspace
                    //         (_, _, "Backspace") => {
                    //             if is_empty { vec!
                    //                 [ EraseEntity(id)
                    //                 , Wander(vm_meta, Direction::Descend, false)
                    //                 ] 
                    //             } else { vec![] }
                    //         }
                    //         // delete
                    //         (_, _, "Delete") => {
                    //             if is_empty { vec!
                    //                 [ EraseEntity(id)
                    //                 ] 
                    //             } else { vec![] }
                    //         }
                    //         // // ctrl released
                    //         // (true, _, "ControlLeft") => vec![Wander(Direction::Stay, false)],
                    //         // (true, _, "ControlRight") => vec![Wander(Direction::Stay, false)],
                            _ => vec![] 
                        }
                    })
                    oninput=link.callback(move |e: InputData| {
                        [EntityUpdate{
                            id, 
                            field: EntityField::Face(e.value)
                        }]
                    })
                />
                { btn_ink(meta.incr_new(), id, link.clone()) }
                // { btn_add(id, id, 0, link.clone()) }
            </div>
        }
    }
}


fn btn_ink(meta: CubeMeta, obj: EntityId, link: ComponentLink<Vase>) -> Html {
    let style = "
        position: absolute; 
        top: 1px;
        right: calc(1 * var(--size-button));
    ";
    btn::ink(meta, obj, style.into(), link)
}

fn btn_add(dude: EntityId, owner: EntityId, idx: usize, link: ComponentLink<Vase>) -> Html {
    let style = "
        position: absolute; 
        top: 1px;
        right: var(--size-button);
    ";
    btn::add(dude, owner, idx, style.into(), link)
}

fn btn_del(id: EntityId, link: ComponentLink<Vase>) -> Html {
    let style = "
        position: absolute; 
        top: 1px;
        right: 0;
    ";
    btn::del(id, style.into(), link)
}

#[cfg(test)]
mod tests {
    // use flow_vessel::EntityIdFactory;
    // use super::*;

    #[test]
    fn update_iter() {
        let correct = vec![0,1,3,4,9,6];
        let mut target = vec![0,4,1,5,5,5,4,6,7];
        target = update_iter_impl(target, &correct);
        println!("correct: {:?}", correct);
        println!("target: {:?}", target);
        assert_eq!(correct, target)
    }

    fn update_iter_impl(mut target: Vec<usize>, correct: &Vec<usize>) -> Vec<usize> {
        for (i, c) in correct.iter().enumerate() {
            println!("i: {}, c: {}, t: {:?}", i, c, target.get(i));
            match target.get(i) {
                Some(id) => {
                    if id != c {
                        let mut rest = target.iter().skip(i+1);
                        if rest.find(|&id| id == c).is_some() {
                            while {
                                let res = target.get(i)
                                    .and_then(|id| {
                                        if id != c {
                                            Some(id)
                                        } else {
                                            None
                                        }
                                    }); 
                                res.is_some()
                            } {
                                target.remove(i);
                            }
                        }
                        if let Some(id) = target.get(i) {
                            if id != c {
                                target.insert(i, c.clone());
                            }
                        }
                    } else {
                        ()
                    }
                },
                None => {
                    target.insert(i, c.clone());
                }
            }
            println!("target: {:?}", target);
        }
        target.truncate(correct.len());
        target
    }

    // #[test]
    // fn update_entity() {
    //     let mut factory = EntityIdFactory::default();
    //     let vec_id: Vec<EntityId> = (0..20)
    //         .map(|_| factory.incr_id())
    //         .collect();
    //     let list = TodoList::new()
    // }
}
