use crate::util::*;
use crate::yew_util::*;
use crate::stockpile::prelude::*;
use crate::view_model::*;

use BranchMessage::*;

// Branch_view

impl BranchModel {
    pub fn branch_view(&self) -> Html {
        // Todo: deal with flow.
        let flow = &self.branch.flow;
        let map = &flow.data;
        html! {
            <div class="branch">
                <div class="branch-group">
                    { for map.iter().map(|(id, _)| self.node_view(id)) }
                    // Debug..
                    <p>{export_json(&map.keys().cloned().collect::<Vec<CubeId>>())}</p>
                </div>
                { self.clearall_button_view() }
            </div>
        }
    }

    fn node_view(&self, id: &CubeId) -> Html {
        let _id = id.clone();
        html! {
            <div class="node">
                // { self.node_status_view(&id) }
                { self.node_input_view(&id) }
            </div>
        }
    }

    // fn node_status_view(&self, id: &CubeId) -> Html {
    //     let id = id.clone();
    //     let vec = ProcessStatus::vec_all();
    //     let status_meta: Vec<(String, String)> = 
    //         vec.iter().map( |x| (
    //             String::from(ProcessStatus::type_src(x)), 
    //             String::from(ProcessStatus::type_str(x))
    //         ) ).collect();
    //     let status_dropdown: Html = 
    //         status_meta.into_iter().map(|(src, des)| {
    //             html! {
    //                 <div title={des.clone()}
    //                     onclick=self.link.callback(move |_| {
    //                         Branchy![UpdateBuffer(des.clone()), WriteProcess(id)]
    //                     })
    //                 > 
    //                     <img src={src} alt="process" /> 
    //                 </div> 
    //             }
    //         }).collect();
    //     html! {
    //         <div class="dropdown"> 
    //             <button class="dropbtn"
    //                 value=self.branch.get(id).process().type_str()
    //             > 
    //                 <img src={self.branch.get(id).process().type_src()} alt="process" />
    //             </button> 
                
    //             <div class="dropdown-content"> 
    //                 { status_dropdown }
    //             </div> 
    //         </div> 
    //     }
    // }

    fn node_input_view(&self, id: &CubeId) -> Html {
        let id = id.clone();
        let is_empty = self.branch.get_cloned(id).name.is_empty();
        html! {
            <input
                type="text"
                ref=self.refs.get(&id).unwrap().clone()
                value=self.branch.get_cloned(id).name
                placeholder="..."
                aria-label="Item"
                onfocus=self.link.callback(move |_| {
                    Branchy![SetFocusId(Some(id))]
                })
                onkeydown=self.link.callback(move |e: KeyboardEvent| {
                    let meta = (e.ctrl_key(), e.shift_key(), e.code());
                    // LOG!("OnKeyDown: {:?}", meta);
                    match (meta.0, meta.1, meta.2.as_str()) { 
                        (false, false, "ArrowUp") => Branchy![Wander(Direction::Ascend, false)], 
                        (false, false, "ArrowDown") => Branchy![Wander(Direction::Descend, false)], 
                        (true, false, "ArrowUp") => Branchy![Wander(Direction::Ascend, true)], 
                        (true, false, "ArrowDown") => Branchy![Wander(Direction::Descend, true)], 
                        (false, false, "ArrowLeft") => Branchy![], 
                        (false, false, "ArrowRight") => Branchy![], 
                        _ => Branchy![]
                    }
                })
                onkeypress=self.link.callback(move |e: KeyboardEvent| {
                    let meta = (e.ctrl_key(), e.shift_key(), e.code());
                    // LOG!("OnKeyPress: {:?}", meta);
                    match (meta.0, meta.1, meta.2.as_str()) { 
                        _ => Branchy![]
                    }
                })
                onkeyup=self.link.callback(move |e: KeyboardEvent| {
                    let meta = (e.ctrl_key(), e.shift_key(), e.code());
                    // LOG!("OnKeyUp: {:?}", meta);
                    match (meta.0, meta.1, meta.2.as_str()) { 
                        // enter
                        (false, false, "Enter") => Branchy![NewCube(Some(id))],
                        // shift+enter
                        (false, true, "Enter") => Branchy![],
                        // backspace
                        (_, _, "Backspace") => {
                            if is_empty { Branchy![EraseCube(id)] }
                            else { Branchy![] }
                        }
                        // delete
                        (_, _, "Delete") => {
                            if is_empty { Branchy![EraseCube(id), EraseCube(id), Wander(Direction::Descend, false)] }
                            else { Branchy![] }
                        }
                        // ctrl released
                        (true, _, "ControlLeft") => Branchy![Wander(Direction::Stay, false)],
                        (true, _, "ControlRight") => Branchy![Wander(Direction::Stay, false)],
                        _ => Branchy![] 
                    }
                })
                oninput=self.link.callback(move |e: InputData| {
                    // LOG!("OnInput: {:?}", e);
                    Branchy![UpdateBuffer(e.value), WriteName(id)]
                })
                // readonly=self.branch.locked
            />
        }
    }

    fn clearall_button_view(&self) -> Html {
        html! {
            <button class="clear-button"
                title="Clear branch."
                ondblclick=self.link.callback(move |_| {
                    Branchy![ClearBranch]
                })
            >{"Clear"}</button>
        }
    }

}

// Branch_src_view

impl BranchModel {
    pub fn src_view(&self) -> Html {
        html! {
            <div class="src">
                <textarea class="src-input"
                    value=self.buffer_str
                    type="text" 
                    oninput=self.link.callback(move |e: InputData| {
                        LOG!("OnInput: {:?}", e);
                        Branchy![UpdateBuffer(e.value)]
                    })
                    spellcheck=false
                />
            </div>
        }
    }
}
