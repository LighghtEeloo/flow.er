use yew::{ComponentLink, Html, InputData, KeyboardEvent, NodeRef, html};
use std::collections::HashMap;
use flow_vessel::{CubeMeta, Entity, EntityField, EntityId, EntityNode, Lint, Process, Symbol, Vessel, cubes::ClauseTree};
use super::{CubeView, Msg::*, Vase, btn};

#[derive(Clone)]
pub struct ClauseNode {
    id: EntityId,
    link: ComponentLink<Vase>
}


impl ClauseNode {
    pub fn view(&self, idx: usize, entity: &Entity, node_ref: NodeRef, owner: EntityId, meta: CubeMeta, indent: usize) -> Html {
        let id = entity.id().clone();
        html! {
            <div class="node">
                { self.symbol_view(idx, &entity, indent) }
                { self.input_view(idx, &entity, node_ref, owner, meta, indent) }
                { btn_block(id, self.link.clone()) }
                { btn_ink(meta.incr_new(), id, self.link.clone()) }
                // { btn_add(id, owner, idx + 1, self.link.clone()) }
                { btn_del(id, self.link.clone()) }
            </div>
        }
    }
    fn symbol_view(&self, idx: usize, entity: &Entity, indent: usize) -> Html {
        let id = entity.id().clone();
        let symbol = match (entity.symbol_toggle, entity.symbol.clone()) {
            (false, Symbol::ProcessTracker(process)) => 
                self.process(id, process),
            (false, Symbol::Linted(lint)) =>
                self.lint(id, idx, lint),
            (true, old) => {
                let product = |old, x: Symbol| {
                    match (old, x.clone()) {
                        (Symbol::Linted(old), Symbol::Linted(_)) => {
                            Symbol::Linted(old)
                        }
                        (Symbol::ProcessTracker(old), Symbol::ProcessTracker(_)) => {
                            Symbol::ProcessTracker(old)
                        }
                        _ => x
                    }
                };
                // list the toggle options
                let contents: Html = html! {
                    <>
                        <div title="toggle-linted"
                            onclick=self.link.callback(move |_| {
                                [ EntityUpdate {
                                    id, 
                                    // id: id.clone(), 
                                    field: EntityField::Symbol(product(old, Symbol::Linted(Lint::default())))
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
                                    field: EntityField::Symbol(product(old, Symbol::ProcessTracker(Process::default())))
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
    fn input_view(&self, idx: usize, entity: &Entity, node_ref: NodeRef, owner: EntityId, meta: CubeMeta, indent: usize) -> Html {
        let indent = indent + 1;
        let id = entity.id().clone();
        let style = 
            format!("width: calc(100% - {} * var(--size-button) - var(--horizontal-margin) * 2);", indent);
        html! {
            <input
                type="text"
                ref=node_ref
                value=entity.face
                style=style
                placeholder="..."
                aria-label="Item"
                // onfocus=self.link.callback(move |_| {
                //     vec![SetFocusId(vm_meta, id)]
                // })
                onkeydown=self.link.callback(move |e: KeyboardEvent| {
                    let key = (e.ctrl_key(), e.shift_key(), e.code());
                    match (key.0, key.1, key.2.as_str()) { 
                //         (false, false, "ArrowUp") => vec!
                //             [Wander(vm_meta, Direction::Ascend, false)], 
                //         (false, false, "ArrowDown") => vec!
                //             [Wander(vm_meta, Direction::Descend, false)], 
                //         (true, false, "ArrowUp") => vec!
                //             [Wander(vm_meta, Direction::Ascend, true)], 
                //         (true, false, "ArrowDown") => vec!
                //             [Wander(vm_meta, Direction::Descend, true)], 
                        (true, true, "ArrowUp") => vec![
                            EntityUp { id },
                            Focus { meta, id }
                        ], 
                        (true, true, "ArrowDown") => vec![
                            EntityDown { id },
                            Focus { meta, id }
                        ], 
                        (true, true, "BracketRight") => vec![
                            EntityDive { id, idx },
                            Focus { meta, id }
                        ], 
                        (true, true, "BracketLeft") => vec![
                            EntityEmerge { id },
                            Focus { meta, id }
                        ], 
                        _ => vec![]
                    }
                })
                onkeyup=self.link.callback(move |e: KeyboardEvent| {
                    let key = (e.ctrl_key(), e.shift_key(), e.code());
                    match (key.0, key.1, key.2.as_str()) { 
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


fn head_view(clause: &ClauseTree, vessel: &Vessel, meta: CubeMeta, link: ComponentLink<Vase>) -> Html {
    let id = clause.obj;
    let entity = vessel.entity(&id).expect("Host doesn't exist.");
    let link = link.clone();
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


impl CubeView for ClauseTree {
    fn view(&self, vessel: &Vessel, meta: CubeMeta, link: ComponentLink<Vase>, ref_map: &HashMap<EntityId, NodeRef>) -> Html {
        let nodes_view: Vec<Html> = vessel.node(&self.obj).expect("node exists")
            .children.iter().enumerate()
            .map(|(idx, &id)| {
                let clause_node = ClauseNode {
                    id,
                    link: link.clone(),
                };
                let node = vessel.node(&clause_node.id).expect("must exist");
                node_view(clause_node, vessel, idx, node, self.obj, ref_map, meta, 0)
            }).collect();
        html! {
            <>
                { head_view(self, vessel, meta, link.clone()) }
                <div class="node-view"> { nodes_view } </div>
            </>
        }
    }
}

fn node_view(
    clause_node: ClauseNode, 
    vessel: &Vessel,
    idx: usize, 
    node: &EntityNode, 
    owner: EntityId,
    ref_map: &HashMap<EntityId, NodeRef>, 
    meta: CubeMeta, 
    indent: usize
) -> Html {
    let node_ref = ref_map.get(node.entity.id()).cloned().unwrap_or_default();
    let clause_node_view = clause_node.view(
        idx, 
        &node.entity, 
        node_ref,
        owner,
        meta,
        indent,
    );
    // Note: no larger than 5.
    let children_view: Vec<Html> = if indent < 5 && !node.entity.blocked {
        node.children.iter().enumerate().map(|(idx, &id)| {
            let clause_node = ClauseNode {
                id,
                link: clause_node.link.clone(),
            };
            let node = vessel.node(&clause_node.id).expect("must exist");
            let owner = node.parent;
            if let Some(owner) = owner {
                node_view(clause_node, vessel, idx, node, owner, ref_map, meta, indent + 1)
            } else {
                html! {}
            }
        }).collect() } else { Vec::new() };
    html! {
        <>
            { clause_node_view }
            { children_view }
        </>
    }
}

fn btn_block(id: EntityId, link: ComponentLink<Vase>) -> Html {
    let style = "
        position: absolute; 
        top: 1px;
        right: calc(2 * var(--size-button));
    ";
    btn::block(id, style.into(), link)
}

fn btn_ink(meta: CubeMeta, obj: EntityId, link: ComponentLink<Vase>) -> Html {
    let style = "
        position: absolute; 
        top: 1px;
        right: calc(1 * var(--size-button));
    ";
    btn::ink(meta, obj, style.into(), link)
}

fn _btn_add(dude: EntityId, owner: EntityId, idx: usize, link: ComponentLink<Vase>) -> Html {
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
