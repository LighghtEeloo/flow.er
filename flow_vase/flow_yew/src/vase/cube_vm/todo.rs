use yew::{ComponentLink, Html, NodeRef, html, InputData};
use flow_vessel::{Entity, EntityField, EntityId, EntityNode, ProcessStatus, Vessel};
use super::{Vase, Msg::*, CubeView};

#[derive(Clone)]
pub struct TodoNode {
    id: EntityId,
    link: ComponentLink<Vase>,
    node_ref: NodeRef,
    buffer_empty: bool,
}


impl TodoNode {
    pub fn new_cube(id: EntityId, link: ComponentLink<Vase>) -> Self {
        Self {
            id,
            link,
            node_ref: NodeRef::default(),
            buffer_empty: true,
        }
    }
    pub fn view(&self, idx: usize, entity: &Entity, owner_id: EntityId) -> Html {
        html! {
            <div class="node">
                { self.status_view(&entity) }
                { self.input_view(idx, &entity, owner_id) }
            </div>
        }
    }
    fn status_view(&self, entity: &Entity) -> Html {
        let id = entity.id().clone();
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
                        onclick=self.link.callback(move |_| {
                            [ EntityUpdate {
                                id, 
                                // id: id.clone(), 
                                field: EntityField::ProcessStatus(process.clone())
                            } ]
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
    fn input_view(&self, idx: usize, entity: &Entity, owner_id: EntityId) -> Html {
        let mut entity = entity.clone();
        let id = entity.id().clone();
        let is_empty = entity.face.is_empty();
        html! {
            <input
                type="text"
                ref=self.node_ref.clone()
                value=entity.face
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


#[derive(Clone)]
pub struct TodoList {
    current: Option<usize>,
    head: TodoNode,
    nodes: Vec<TodoNode>
}

impl TodoList {
    pub fn new_cube(entity_node: &EntityNode, current: Option<usize>, link: ComponentLink<Vase>) -> CubeView {
        let mut nodes = Vec::new();
        let id = entity_node.entity.id();
        for id in entity_node.children.iter() {
            nodes.push(TodoNode::new_cube(id.clone(), link.clone()))
        }
        let todo = Self {
            current,
            head: TodoNode::new_cube(id.clone(), link),
            nodes
        };
        CubeView::TodoList {
            todo
        }
    }
    pub fn update(&mut self, entity_node: &EntityNode) {
        let link = self.head.link.clone();
        let correct = &entity_node.children;
        let target = self.nodes.clone();
        self.nodes = TodoList::update_iter_impl(target, correct, link);
    }
    fn update_iter_impl(mut target: Vec<TodoNode>, correct: &Vec<EntityId>, link: ComponentLink<Vase>) -> Vec<TodoNode> {
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
                    let node = TodoNode::new_cube(c.clone(), link.clone());
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
            <div class="node-view"> { nodes_view } </div>
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
