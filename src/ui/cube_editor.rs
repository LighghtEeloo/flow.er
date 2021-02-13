use crate::yew_util::*;
use crate::stockpile::prelude::*;
use crate::ui::*;

use CubeMessage::*;

// Cube_new_view

impl Model {
    pub fn cube_new_view(&self) -> Html {
        html! {
            <div class="cube-new">
            { self.cube_new_input_view() }
            </div>
        }
    }
    pub fn cube_new_input_view(&self) -> Html {
        html! {
            <div class="cube-input">
                <input
                    type="text"
                    ref=self.ref_cube_name.clone()
                    placeholder="Enter new proj name."
                    oninput=self.link.callback(move |e: InputData| {
                        LOG!("OnInput - new: {:?}", e);
                        Cubey![UpdateBuffer(e.value)]
                    })
                    onkeypress=self.link.callback(move |e: KeyboardEvent| {
                        LOG!("OnKeyPress: {:?}", e);
                        if e.key() == "Enter" { Cubey![NewCube] } else { Cubey![] }
                    })
                />
                <div class="dash-line"></div>
            </div>
        }
    }
}

// Cube_view

impl Model {
    pub fn cube_view(&self) -> Html {
        let relation = &self.cube.relation;
        let vec = &relation.data;
        html! {
            <div class="cube">
                { self.cube_input_view() }
                <div class="node-group">
                    { self.add_button_view(vec![]) }
                    { for vec.iter().map(|id| self.node_view(id)) }
                </div>
                { self.clearall_button_view() }
                // { self.src_view_button_view() }
            </div>
        }
    }

    fn cube_input_view(&self) -> Html {
        html! {
            <div class="cube-input">
                <input
                    type="text"
                    ref=self.ref_cube_name.clone()
                    placeholder="Enter new proj name."
                    value=self.cube.name
                    onfocus=self.link.callback(move |_| {
                        Cubey![SetFocusId(None)]
                    })
                    oninput=self.link.callback(move |e: InputData| {
                        LOG!("OnInput: {:?}", e);
                        Cubey![UpdateBuffer(e.value), WriteCubeName]
                    })
                    onkeydown=self.link.callback(move |e: KeyboardEvent| {
                        let meta = (e.ctrl_key(), e.shift_key(), e.code());
                        LOG!("OnKeyDown: {:?}", meta);
                        match (meta.0, meta.1, meta.2.as_str()) { 
                            (false, false, "ArrowDown") => Cubey![Wander(Direction::Descend, false)], 
                            _ => Cubey![] 
                        }
                    })
                    onkeyup=self.link.callback(move |e: KeyboardEvent| {
                        LOG!("OnKeyUp: {:?}", e);
                        if e.key() == "Enter" { Cubey![NewNode(vec![])] } else { Cubey![] }
                    })
                />
                <div class="dash-line"></div>
            </div>
        }
    }
    
    fn node_view(&self, id: &EntryId) -> Html {
        let id = id.clone();
        html! {
            <div class="node">
                { self.node_status_view(&id) }
                { self.node_input_view(&id) }
                { self.add_button_view(vec![id]) }
                { self.erase_button_view(&id) }
            </div>
        }
    }

    fn node_status_view(&self, id: &EntryId) -> Html {
        let id = id.clone();
        let vec = ProcessStatus::vec_all();
        let status_meta: Vec<(String, String)> = 
            vec.iter().map( |x| (
                String::from(ProcessStatus::type_src(x)), 
                String::from(ProcessStatus::type_str(x))
            ) ).collect();
        let status_dropdown: Html = 
            status_meta.into_iter().map(|(src, des)| {
                html! {
                    <ul title={des.clone()}
                        onclick=self.link.callback(move |_| {
                            Cubey![UpdateBuffer(des.clone()), WriteProcess(id)]
                        })
                    > 
                        <img src={src} /> 
                    </ul> 
                }
            }).collect();
        html! {
            <div class="dropdown"> 
                <button class="dropbtn"
                    value=self.cube.get(id).process().type_str()
                > 
                    <img src={self.cube.get(id).process().type_src()} />
                </button> 
                
                <div class="dropdown-content"> 
                    { status_dropdown }
                </div> 
            </div> 
        }
    }

    fn node_input_view(&self, id: &EntryId) -> Html {
        let id = id.clone();
        let is_empty = self.cube.get(id).face().is_empty();
        html! {
            <input
                type="text"
                ref=self.refs.get(&id).unwrap().clone()
                value=self.cube.get(id).face()
                placeholder="..."
                onfocus=self.link.callback(move |_| {
                    Cubey![SetFocusId(Some(id))]
                })
                onkeydown=self.link.callback(move |e: KeyboardEvent| {
                    let meta = (e.ctrl_key(), e.shift_key(), e.code());
                    LOG!("OnKeyDown: {:?}", meta);
                    match (meta.0, meta.1, meta.2.as_str()) { 
                        (false, false, "ArrowUp") => Cubey![Wander(Direction::Ascend, false)], 
                        (false, false, "ArrowDown") => Cubey![Wander(Direction::Descend, false)], 
                        (true, false, "ArrowUp") => Cubey![Wander(Direction::Ascend, true)], 
                        (true, false, "ArrowDown") => Cubey![Wander(Direction::Descend, true)], 
                        (false, false, "ArrowLeft") => Cubey![], 
                        (false, false, "ArrowRight") => Cubey![], 
                        _ => Cubey![]
                    }
                })
                onkeypress=self.link.callback(move |e: KeyboardEvent| {
                    let meta = (e.ctrl_key(), e.shift_key(), e.code());
                    LOG!("OnKeyPress: {:?}", meta);
                    match (meta.0, meta.1, meta.2.as_str()) { 
                        _ => Cubey![]
                    }
                })
                onkeyup=self.link.callback(move |e: KeyboardEvent| {
                    let meta = (e.ctrl_key(), e.shift_key(), e.code());
                    LOG!("OnKeyUp: {:?}", meta);
                    match (meta.0, meta.1, meta.2.as_str()) { 
                        // enter
                        (false, false, "Enter") => Cubey![NewNode(vec![id])],
                        // shift+enter
                        (false, true, "Enter") => Cubey![],
                        // Todo: Delay.
                        // backspace
                        (_, _, "Backspace") => {
                            if is_empty { Cubey![EraseNode(id)] }
                            // if is_empty { vec![EraseNode(id),Wander(Direction::Ascend)] }
                            else { Cubey![] }
                        }
                        // delete
                        (_, _, "Delete") => {
                            if is_empty { Cubey![EraseNode(id)] }
                            else { Cubey![] }
                        }
                        // ctrl released
                        (true, _, "ControlLeft") => Cubey![Wander(Direction::Stay, false)],
                        (true, _, "ControlRight") => Cubey![Wander(Direction::Stay, false)],
                        _ => Cubey![] 
                    }
                })
                oninput=self.link.callback(move |e: InputData| {
                    LOG!("OnInput: {:?}", e);
                    Cubey![UpdateBuffer(e.value), WriteFace(id)]
                })
                readonly=self.cube.locked
            />
        }
    }

    fn add_button_view(&self, id_vec: Vec<EntryId>) -> Html {
        html! {
            <button class="add-button"
                title="New node."
                onclick=self.link.callback(move |_| {
                    LOG!("OnClick.");
                    Cubey![NewNode(id_vec.clone())]
                })
            >{"+"}</button>
        }
    }

    fn erase_button_view(&self, id: &EntryId) -> Html {
        let id = id.clone();
        html! {
            <button class="del-button"
                title="Erase node."
                onclick=self.link.callback(move |_| {
                    LOG!("OnClick.");
                    Cubey![EraseNode(id)]
                })
            >{" - "}</button>
        }
    }

    fn clearall_button_view(&self) -> Html {
        html! {
            <button class="clear-button"
                title="Clear cube."
                ondblclick=self.link.callback(move |_| {
                    Cubey![ClearCube]
                })
            >{"Clear"}</button>
        }
    }

}

// Cube_src_view

impl Model {
    pub fn cube_src_view(&self) -> Html {
        html! {
            <div class="cube-src">
                <textarea class="cube-src-input"
                    value=self.buffer_str
                    type="text" 
                    oninput=self.link.callback(move |e: InputData| {
                        LOG!("OnInput: {:?}", e);
                        Cubey![UpdateBuffer(e.value)]
                    })
                    spellcheck=false
                />
            </div>
        }
    }
}
