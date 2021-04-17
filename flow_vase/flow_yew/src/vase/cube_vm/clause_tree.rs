use yew::{Component, ComponentLink, NodeRef, Properties, html, Html, *};
use std::collections::HashMap;
use flow_vessel::{ClauseTreeCore, CubeMeta, EntityId, EntityNode, Tube::{self, *}, *};
use super::{Vase, btn};



pub struct ClauseTree {
    props: Props,
    ref_map: HashMap<EntityId, NodeRef>,
    link: ComponentLink<Self>
}

#[derive(Properties, Clone)]
pub struct Props {
    pub core: ClauseTreeCore,
    pub meta: CubeMeta,
    pub link_tube: ComponentLink<Vase>
}

pub enum Msg {
    Tube (Tube)
}

impl Component for ClauseTree {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: yew::ComponentLink<Self>) -> Self {
        let ref_map = props.core.node_map.keys()
            .map(|x| {
                (x.clone(), NodeRef::default())
            }).collect();
        Self {
            props,
            ref_map,
            link
        }
    }

    fn update(&mut self, msg: Self::Message) -> yew::ShouldRender {
        match msg {
            Msg::Tube(tube) => {
                // Todo:: tube callback.
                // self.props.link_tube.callback(|_| tube).emit(());
                todo!()
            }
        }
        true
    }

    fn change(&mut self, props: Self::Properties) -> yew::ShouldRender {
        self.props = props;
        true
    }

    fn view(&self) -> yew::Html {
        let core = &self.props.core;
        let arg_head = (self.props.meta, self.props.link_tube.clone());
        let arg_node = (self.props.meta, self.props.link_tube.clone());
        html! {
            <>
                { core.head_view(head_view, arg_head) }
                <div class="node-view"> { core.node_view(node_view, combinator, arg_node, |x| {x}) } </div>
            </>
        }
    }
}

fn combinator(h: Html, vec: Vec<Html>) -> Html {
    html! {
        <>
            { h }
            { vec }
        </>
    }
}


fn head_view(node: &EntityNode, (meta, link): (CubeMeta, ComponentLink<Vase>)) -> Html {
    let id = node.id().clone();
    let entity = node.entity.clone();
    let link = link.clone();
    html! {
        <div class="head">
            <input
                type="Text"
                placeholder="An arbitrary node."
                aria-label="Arbitrary Node"
                value=entity.face
                onkeyup=link.callback(move |e: KeyboardEvent| {
                    let key = (e.ctrl_key(), e.shift_key(), e.code());
                    match (key.0, key.1, key.2.as_str()) { 
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

fn node_view(node: &EntityNode, (meta, link): (CubeMeta, ComponentLink<Vase>)) -> Html {
    todo!()
}

#[derive(Clone)]
pub struct ClauseNode {
    id: EntityId,
    node_ref: NodeRef,
    link: ComponentLink<Vase>
}


// impl ClauseNode {
//     pub fn view(&self, idx: usize, entity: &Entity, node_ref: NodeRef, owner: EntityId, meta: CubeMeta, indent: usize) -> Html {
//         let id = entity.id().clone();
//         html! {
//             <div class="node">
//                 { self.symbol_view(idx, &entity, indent) }
//                 { self.input_view(idx, &entity, node_ref, owner, meta, indent) }
//                 { btn_block(id, self.link.clone()) }
//                 { btn_ink(meta.incr_new(), id, self.link.clone()) }
//                 // { btn_add(id, owner, idx + 1, self.link.clone()) }
//                 { btn_del(id, self.link.clone()) }
//             </div>
//         }
//     }
//     fn symbol_view(&self, idx: usize, entity: &Entity, indent: usize) -> Html {
//         let id = entity.id().clone();
//         let symbol = match (entity.symbol_toggle, entity.symbol.clone()) {
//             (false, Symbol::ProcessTracker(process)) => 
//                 self.process(id, process),
//             (false, Symbol::Linted(lint)) =>
//                 self.lint(id, idx, lint),
//             (true, old) => {
//                 let product = |old, x: Symbol| {
//                     match (old, x.clone()) {
//                         (Symbol::Linted(old), Symbol::Linted(_)) => {
//                             Symbol::Linted(old)
//                         }
//                         (Symbol::ProcessTracker(old), Symbol::ProcessTracker(_)) => {
//                             Symbol::ProcessTracker(old)
//                         }
//                         _ => x
//                     }
//                 };
//                 // list the toggle options
//                 let contents: Html = html! {
//                     <>
//                         <div title="toggle-linted"
//                             onclick=self.link.callback(move |_| {
//                                 [ EntityUpdate {
//                                     id, 
//                                     // id: id.clone(), 
//                                     field: EntityField::Symbol(product(old, Symbol::Linted(Lint::default())))
//                                 } ]
//                             })
//                         >
//                             <span>{Lint::default().display(0)}</span>
//                         </div>
//                         <div title="toggle-process-tracker"
//                             onclick=self.link.callback(move |_| {
//                                 [ EntityUpdate {
//                                     id, 
//                                     // id: id.clone(), 
//                                     field: EntityField::Symbol(product(old, Symbol::ProcessTracker(Process::default())))
//                                 } ]
//                             })
//                         >
//                             <img src={Process::type_src(&Process::default())} alt="process" /> 
//                         </div>
//                     </>
//                 };
//                 html! {
//                     <div class="dropdown-content" style="display: block">
//                         { contents }
//                     </div>
//                 }
//             }
//         };
//         let style = 
//             format!("left: calc({} * var(--size-button) + {}px);", indent, indent);
//         html! {
//             <div class="symbol" style=style> 
//                 { symbol }
//             </div> 
//         }
//     }
//     fn input_view(&self, idx: usize, entity: &Entity, node_ref: NodeRef, owner: EntityId, meta: CubeMeta, indent: usize) -> Html {
//         let indent = indent + 1;
//         let id = entity.id().clone();
//         let style = 
//             format!("width: calc(100% - {} * var(--size-button) - var(--horizontal-margin) * 2);", indent);
//         html! {
//             <input
//                 type="text"
//                 ref=node_ref
//                 value=entity.face
//                 style=style
//                 placeholder="..."
//                 aria-label="Item"
//                 // onfocus=self.link.callback(move |_| {
//                 //     vec![SetFocusId(vm_meta, id)]
//                 // })
//                 onkeydown=self.link.callback(move |e: KeyboardEvent| {
//                     let key = (e.ctrl_key(), e.shift_key(), e.code());
//                     match (key.0, key.1, key.2.as_str()) { 
//                 //         (false, false, "ArrowUp") => vec!
//                 //             [Wander(vm_meta, Direction::Ascend, false)], 
//                 //         (false, false, "ArrowDown") => vec!
//                 //             [Wander(vm_meta, Direction::Descend, false)], 
//                 //         (true, false, "ArrowUp") => vec!
//                 //             [Wander(vm_meta, Direction::Ascend, true)], 
//                 //         (true, false, "ArrowDown") => vec!
//                 //             [Wander(vm_meta, Direction::Descend, true)], 
//                         (true, true, "ArrowUp") => vec![
//                             EntityUp { id },
//                             Focus { meta, id }
//                         ], 
//                         (true, true, "ArrowDown") => vec![
//                             EntityDown { id },
//                             Focus { meta, id }
//                         ], 
//                         (true, true, "BracketRight") => vec![
//                             EntityDive { id, idx },
//                             Focus { meta, id }
//                         ], 
//                         (true, true, "BracketLeft") => vec![
//                             EntityEmerge { id },
//                             Focus { meta, id }
//                         ], 
//                         _ => vec![]
//                     }
//                 })
//                 onkeyup=self.link.callback(move |e: KeyboardEvent| {
//                     let key = (e.ctrl_key(), e.shift_key(), e.code());
//                     match (key.0, key.1, key.2.as_str()) { 
//                         // enter
//                         (false, false, "Enter") => vec!
//                             [ EntityAdd { dude: id, owner, idx: idx+1 }
//                             // , Wander(vm_meta, Direction::Descend, false)
//                             ],
//                 //         // // shift+enter
//                 //         // (false, true, "Enter") => vec![],
//                 //         // backspace
//                 //         (_, _, "Backspace") => {
//                 //             if is_empty { vec!
//                 //                 [ EraseEntity(id)
//                 //                 , Wander(vm_meta, Direction::Descend, false)
//                 //                 ] 
//                 //             } else { vec![] }
//                 //         }
//                 //         // delete
//                 //         (_, _, "Delete") => {
//                 //             if is_empty { vec!
//                 //                 [ EraseEntity(id)
//                 //                 ] 
//                 //             } else { vec![] }
//                 //         }
//                 //         // // ctrl released
//                 //         // (true, _, "ControlLeft") => vec![Wander(Direction::Stay, false)],
//                 //         // (true, _, "ControlRight") => vec![Wander(Direction::Stay, false)],
//                         _ => vec![] 
//                     }
//                 })
//                 oninput=self.link.callback(move |e: InputData| {
//                     [ EntityUpdate {
//                         id, 
//                         field: EntityField::Face(e.value)
//                     } ]
//                 })
//                 // readonly=self.locked
//             />
//         }
//     }
// }

// // symbol view
// impl ClauseNode {
//     fn process(&self, id: EntityId, process: Process) -> Html {
//         let process_meta: Vec<(String, String, Process)> = 
//         Process::vec_all().iter().map( |x| (
//             String::from(Process::type_src(x)), 
//             String::from(Process::type_str(x)),
//             x.clone()
//         ) ).collect();
//         let dropdown: Html = 
//             process_meta.into_iter().map(|(src, des, process)| {
//                 html! {
//                     <div title={des.clone()}
//                         onclick=self.link.callback(move |_| {
//                             [ EntityUpdate {
//                                 id, 
//                                 // id: id.clone(), 
//                                 field: EntityField::Symbol(Symbol::ProcessTracker(process.clone()))
//                             } ]
//                         })
//                     > 
//                         <img src={src} alt="process" /> 
//                     </div> 
//                 }
//             }).collect();
//         html! {    
//             <>
//                 <button class="dropbtn process"
//                     value=process.type_str()
//                     onclick=self.link.callback(move|_| {
//                         [EntityUpdate{
//                             id,
//                             field: EntityField::SymbolToggle
//                         }]
//                     })
//                 > 
//                     <img src={process.type_src()} alt="process" />
//                 </button> 
                
//                 <div class="dropdown-content"> 
//                     {dropdown}
//                 </div> 
//             </>
//         }
//     }
//     fn lint(&self, id: EntityId, idx: usize, lint: Lint) -> Html {
//         let text = lint.display(idx);
//         let lint_meta: Vec<(String, Lint)> = Lint::vec_all().into_iter().map(|x|
//             (x.display(0), x)
//         ).collect();
//         let dropdown: Html = 
//             lint_meta.into_iter().map(|(text, lint)| {
//                 html! {
//                     <div title={lint.clone().type_str()}
//                         onclick=self.link.callback(move |_| {
//                             [ EntityUpdate {
//                                 id, 
//                                 // id: id.clone(), 
//                                 field: EntityField::Symbol(Symbol::Linted(lint.clone()))
//                             } ]
//                         })
//                     > 
//                         <span> {text} </span>
//                     </div> 
//                 }
//             }).collect();
//         html! {
//             <>
//                 <button class="dropbtn lint"
//                     onclick=self.link.callback(move|_| {
//                         [EntityUpdate{
//                             id,
//                             field: EntityField::SymbolToggle
//                         }]
//                     })
//                 > 
//                     <div class="symbol-text"> {text} </div>
//                 </button> 

//                 <div class="dropdown-content"> 
//                     {dropdown}
//                 </div> 
//             </>
//         }
//     }
// }



// fn btn_block(id: EntityId, link: ComponentLink<Vase>) -> Html {
//     let style = "
//         position: absolute; 
//         top: 1px;
//         right: calc(2 * var(--size-button));
//     ";
//     btn::block(id, style.into(), link)
// }

fn btn_ink(meta: CubeMeta, obj: EntityId, link: ComponentLink<Vase>) -> Html {
    let style = "
        position: absolute; 
        top: 1px;
        right: calc(1 * var(--size-button));
    ";
    btn::ink(meta, obj, style.into(), link)
}

// fn _btn_add(dude: EntityId, owner: EntityId, idx: usize, link: ComponentLink<Vase>) -> Html {
//     let style = "
//         position: absolute; 
//         top: 1px;
//         right: var(--size-button);
//     ";
//     btn::add(dude, owner, idx, style.into(), link)
// }

// fn btn_del(id: EntityId, link: ComponentLink<Vase>) -> Html {
//     let style = "
//         position: absolute; 
//         top: 1px;
//         right: 0;
//     ";
//     btn::del(id, style.into(), link)
// }
