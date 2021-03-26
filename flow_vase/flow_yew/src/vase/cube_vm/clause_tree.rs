use yew::{ComponentLink, Html, NodeRef, html, InputData};
use flow_vessel::{Entity, EntityField, EntityId, EntityNode, Lint, Process, Symbol, Vessel};
use super::{Vase, Msg::*, CubeView};

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
    pub fn view(&self, idx: usize, entity: &Entity, owner: EntityId) -> Html {
        let id = entity.id().clone();
        let btn_add = html! {
            <button class="btn-add" style="position: absolute; right: 10%"
                onclick=self.link.callback(move |_| {
                    [EntityAdd{
                        owner,
                        idx: idx + 1
                    }]
                })
            >{"+"}</button>
        };
        let btn_del = html! {
            <button class="btn-del" style="position: absolute; left: 90%"
                onclick=self.link.callback(move |_| {
                    [EntityDelete{id}]
                })
            >{"x"}</button>
        };
        html! {
            <div class="node">
                { self.symbol_view(idx, &entity) }
                { self.input_view(idx, &entity, owner) }
                { btn_del }
                { btn_add }
            </div>
        }
    }
    fn symbol_view(&self, idx: usize, entity: &Entity) -> Html {
        let indent_base = 0;
        let indent = indent_base;
        let id = entity.id().clone();
        let symbol = match entity.symbol.clone() {
            Symbol::ProcessTracker(process) => 
                self.process(id, process),
            Symbol::Linted(lint) =>
                self.lint(idx, lint),
            _ => html!{<></>}
        };
        let style = 
            format!("left: calc({} * var(--size-button) + {}px);", indent, indent);
        html! {
            <div class="symbol" style=style> 
                { symbol }
            </div> 
        }
    }
    fn input_view(&self, idx: usize, entity: &Entity, owner_id: EntityId) -> Html {
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
                // onkeydown=self.link.callback(move |e: KeyboardEvent| {
                //     let meta = (e.ctrl_key(), e.shift_key(), e.code());
                //     match (meta.0, meta.1, meta.2.as_str()) { 
                //         (false, false, "ArrowUp") => vec!
                //             [Wander(vm_meta, Direction::Ascend, false)], 
                //         (false, false, "ArrowDown") => vec!
                //             [Wander(vm_meta, Direction::Descend, false)], 
                //         (true, false, "ArrowUp") => vec!
                //             [Wander(vm_meta, Direction::Ascend, true)], 
                //         (true, false, "ArrowDown") => vec!
                //             [Wander(vm_meta, Direction::Descend, true)], 
                //         // (false, false, "ArrowLeft") => vec![], 
                //         // (false, false, "ArrowRight") => vec![], 
                //         _ => vec![]
                //     }
                // })
                // onkeyup=self.link.callback(move |e: KeyboardEvent| {
                //     let meta = (e.ctrl_key(), e.shift_key(), e.code());
                //     match (meta.0, meta.1, meta.2.as_str()) { 
                //         // enter
                //         (false, false, "Enter") => vec!
                //             [ AddEntity(FlowLink::new_descend_index(owner_id, idx + 1))
                //             , Wander(vm_meta, Direction::Descend, false)
                //             ],
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
                //         _ => vec![] 
                //     }
                // })
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
                <button class="dropbtn"
                    value=process.type_str()
                > 
                    <img src={process.type_src()} alt="process" />
                </button> 
                
                <div class="dropdown-content"> 
                    {dropdown}
                </div> 
            </>
        }
    }
    fn lint(&self, idx: usize, lint: Lint) -> Html {
        let text = lint.display(idx);
        html! {
            <>
            <div class="symbol-text"> {text} </div>
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
    pub fn update(&mut self, entity_node: &EntityNode) {
        let link = self.head.link.clone();
        let correct = &entity_node.children;
        let target = self.nodes.clone();
        self.nodes = ClauseTree::update_iter_impl(target, correct, link);
    }
    fn update_iter_impl(mut target: Vec<ClauseNode>, correct: &Vec<EntityId>, link: ComponentLink<Vase>) -> Vec<ClauseNode> {
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
    pub fn view(&self, vessel: &Vessel) -> Html {
        let nodes_view: Vec<Html> = self.nodes.iter().enumerate()
            .map(|(idx, node)| {
                node.view(
                    idx, 
                    vessel.entity_get(&node.id).expect("must exist"), 
                    self.head.id
                )
            }).collect();
        html! {
            <>
                { self.head_view(vessel) }
                <div class="node-view"> { nodes_view } </div>
            </>
        }
    }

    fn head_view(&self, vessel: &Vessel) -> Html {
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
                    oninput=link.callback(move |e: InputData| {
                        [EntityUpdate{
                            id, 
                            field: EntityField::Face(e.value)
                        }]
                    })
                />
            </div>
        }
    }
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
