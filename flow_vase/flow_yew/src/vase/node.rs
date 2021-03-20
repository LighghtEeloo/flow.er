use yew::{html, ComponentLink, Html};
use super::{Vase, Entity, EntityId};

pub struct Node {
    link: ComponentLink<Vase>
}


// impl Node {
//     fn node_view(&self, idx: usize, entity: &Entity, owner_id: EntityId, vm_meta: VMMeta, link: &ComponentLink<Vase>) -> Html {
//         html! {
//             <div class="node">
//                 { self.node_status_view(&entity, link) }
//                 { self.node_input_view(idx, &entity, owner_id, vm_meta, link) }
//             </div>
//         }
//     }
//     fn node_status_view(&self, entity: &Entity, link: &ComponentLink<Vase>) -> Html {
//         let id = entity.id();
//         let vec = ProcessStatus::vec_all();
//         let status_meta: Vec<(String, String, ProcessStatus)> = 
//             vec.iter().map( |x| (
//                 String::from(ProcessStatus::type_src(x)), 
//                 String::from(ProcessStatus::type_str(x)),
//                 x.clone()
//             ) ).collect();
//         let status_dropdown: Html = 
//             status_meta.into_iter().map(|(src, des, process)| {
//                 html! {
//                     <div title={des.clone()}
//                         onclick=link.callback(move |_| {
//                             Vasey![WriteEntity(id, EntityField::ProcessStatus(process.clone()))]
//                         })
//                     > 
//                         <img src={src} alt="process" /> 
//                     </div> 
//                 }
//             }).collect();
//         html! {
//             <div class="dropdown"> 
//                 <button class="dropbtn"
//                     value=entity.process.type_str()
//                 > 
//                     <img src={entity.process.type_src()} alt="process" />
//                 </button> 
                
//                 <div class="dropdown-content"> 
//                     { status_dropdown }
//                 </div> 
//             </div> 
//         }
//     }
//     fn node_input_view(&self, idx: usize, entity: &Entity, owner_id: EntityId, vm_meta: VMMeta, link: &ComponentLink<Vase>) -> Html {
//         let mut entity = entity.clone();
//         let id = entity.id();
//         let is_empty = entity.face.is_empty();
//         html! {
//             <input
//                 type="text"
//                 ref=self.refs.get(&id).unwrap().clone()
//                 value=entity.face
//                 placeholder="..."
//                 aria-label="Item"
//                 onfocus=link.callback(move |_| {
//                     Vasey![SetFocusId(vm_meta, id)]
//                 })
//                 onkeydown=link.callback(move |e: KeyboardEvent| {
//                     let meta = (e.ctrl_key(), e.shift_key(), e.code());
//                     match (meta.0, meta.1, meta.2.as_str()) { 
//                         (false, false, "ArrowUp") => Vasey!
//                             [Wander(vm_meta, Direction::Ascend, false)], 
//                         (false, false, "ArrowDown") => Vasey!
//                             [Wander(vm_meta, Direction::Descend, false)], 
//                         (true, false, "ArrowUp") => Vasey!
//                             [Wander(vm_meta, Direction::Ascend, true)], 
//                         (true, false, "ArrowDown") => Vasey!
//                             [Wander(vm_meta, Direction::Descend, true)], 
//                         // (false, false, "ArrowLeft") => Vasey![], 
//                         // (false, false, "ArrowRight") => Vasey![], 
//                         _ => Vasey![]
//                     }
//                 })
//                 // onkeypress=link.callback(move |e: KeyboardEvent| {
//                 //     let meta = (e.ctrl_key(), e.shift_key(), e.code());
//                 //     match (meta.0, meta.1, meta.2.as_str()) { 
//                 //         _ => Vasey![]
//                 //     }
//                 // })
//                 onkeyup=link.callback(move |e: KeyboardEvent| {
//                     let meta = (e.ctrl_key(), e.shift_key(), e.code());
//                     match (meta.0, meta.1, meta.2.as_str()) { 
//                         // enter
//                         (false, false, "Enter") => Vasey!
//                             [ AddEntity(FlowLink::new_descend_index(owner_id, idx + 1))
//                             , Wander(vm_meta, Direction::Descend, false)
//                             ],
//                         // // shift+enter
//                         // (false, true, "Enter") => Vasey![],
//                         // backspace
//                         (_, _, "Backspace") => {
//                             if is_empty { Vasey!
//                                 [ EraseEntity(id)
//                                 , Wander(vm_meta, Direction::Descend, false)
//                                 ] 
//                             } else { Vasey![] }
//                         }
//                         // delete
//                         (_, _, "Delete") => {
//                             if is_empty { Vasey!
//                                 [ EraseEntity(id)
//                                 ] 
//                             } else { Vasey![] }
//                         }
//                         // // ctrl released
//                         // (true, _, "ControlLeft") => Vasey![Wander(Direction::Stay, false)],
//                         // (true, _, "ControlRight") => Vasey![Wander(Direction::Stay, false)],
//                         _ => Vasey![] 
//                     }
//                 })
//                 oninput=link.callback(move |e: InputData| {
//                     Vasey![WriteEntity(id, EntityField::Face(e.value))]
//                 })
//                 readonly=self.locked
//             />
//         }
//     }
// }
