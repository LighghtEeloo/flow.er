use crate::ui::*;
use crate::yew_util::*;
use crate::cube::prelude::*;

impl Model {
    pub fn cube_new_view(&self) -> Html {
        html! {
            <div class="cube-new">
            { self.cube_new_input_view() }
            </div>
        }
    }
    pub fn cube_new_input_view(&self) -> Html {
        use Msg::*;
        html! {
            <div class="cube-input">
                <input
                    type="text"
                    ref=self.ref_name.clone()
                    placeholder="Enter new proj name."
                    oninput=self.link.callback(move |e: InputData| {
                        LOG!("OnInput - new: {:?}", e);
                        [UpdateBuffer(e.value)]
                    })
                    onkeypress=self.link.callback(move |e: KeyboardEvent| {
                        LOG!("OnKeyPress: {:?}", e);
                        if e.key() == "Enter" { vec![NewCube] } else { vec![] }
                    })
                />
                <div class="dash-line"></div>
            </div>
        }
    }

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
            </div>
        }
    }

    fn cube_input_view(&self) -> Html {
        use Msg::*;
        html! {
            <div class="cube-input">
                <input
                    type="text"
                    ref=self.ref_name.clone()
                    placeholder="Enter new proj name."
                    value=self.cube.name
                    onfocus=self.link.callback(move |_| {
                        [SetFocusId(None)]
                    })
                    oninput=self.link.callback(move |e: InputData| {
                        LOG!("OnInput: {:?}", e);
                        [UpdateBuffer(e.value), WriteCubeName]
                    })
                    onkeydown=self.link.callback(move |e: KeyboardEvent| {
                        let meta = (e.ctrl_key(), e.shift_key(), e.code());
                        LOG!("OnKeyDown: {:?}", meta);
                        match (meta.0, meta.1, meta.2.as_str()) { 
                            (false, false, "ArrowDown") => vec![Wander(Direction::Descend, false)], 
                            _ => vec![] 
                        }
                    })
                    onkeyup=self.link.callback(move |e: KeyboardEvent| {
                        LOG!("OnKeyUp: {:?}", e);
                        if e.key() == "Enter" { vec![NewNode(vec![])] } else { vec![] }
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
        use Msg::*;
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
                            [UpdateBuffer(des.clone()), WriteProcess(id)]
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
        use Msg::*;
        let id = id.clone();
        let is_empty = self.cube.get(id).face().is_empty();
        html! {
            <input
                type="text"
                ref=self.refs.get(&id).unwrap().clone()
                value=self.cube.get(id).face()
                placeholder="..."
                onfocus=self.link.callback(move |_| {
                    [SetFocusId(Some(id))]
                })
                onkeydown=self.link.callback(move |e: KeyboardEvent| {
                    let meta = (e.ctrl_key(), e.shift_key(), e.code());
                    LOG!("OnKeyDown: {:?}", meta);
                    match (meta.0, meta.1, meta.2.as_str()) { 
                        (false, false, "ArrowUp") => vec![Wander(Direction::Ascend, false)], 
                        (false, false, "ArrowDown") => vec![Wander(Direction::Descend, false)], 
                        (true, false, "ArrowUp") => vec![Wander(Direction::Ascend, true)], 
                        (true, false, "ArrowDown") => vec![Wander(Direction::Descend, true)], 
                        (false, false, "ArrowLeft") => vec![], 
                        (false, false, "ArrowRight") => vec![], 
                        _ => vec![] 
                    }
                })
                onkeypress=self.link.callback(move |e: KeyboardEvent| {
                    let meta = (e.ctrl_key(), e.shift_key(), e.code());
                    LOG!("OnKeyPress: {:?}", meta);
                    match (meta.0, meta.1, meta.2.as_str()) { 
                        _ => vec![] 
                    }
                })
                onkeyup=self.link.callback(move |e: KeyboardEvent| {
                    let meta = (e.ctrl_key(), e.shift_key(), e.code());
                    LOG!("OnKeyUp: {:?}", meta);
                    match (meta.0, meta.1, meta.2.as_str()) { 
                        // enter
                        (false, false, "Enter") => vec![NewNode(vec!(id))],
                        // shift+enter
                        (false, true, "Enter") => vec![],
                        // Todo: Delay.
                        // backspace
                        (_, _, "Backspace") => {
                            if is_empty { vec![EraseNode(id)] }
                            // if is_empty { vec![EraseNode(id),Wander(Direction::Ascend)] }
                            else { vec![] }
                        }
                        // delete
                        (_, _, "Delete") => {
                            if is_empty { vec![EraseNode(id)] }
                            else { vec![] }
                        }
                        // ctrl released
                        (true, _, "ControlLeft") => vec![Wander(Direction::Stay, false)],
                        (true, _, "ControlRight") => vec![Wander(Direction::Stay, false)],
                        _ => vec![] 
                    }
                })
                oninput=self.link.callback(move |e: InputData| {
                    LOG!("OnInput: {:?}", e);
                    [UpdateBuffer(e.value), WriteFace(id)]
                })
                readonly=if self.cube.locked { true } else { false }
            />
        }
    }

    fn add_button_view(&self, id_vec: Vec<EntryId>) -> Html {
        use Msg::*;
        html! {
            <button class="add-button"
                title="New node."
                onclick=self.link.callback(move |_| {
                    LOG!("OnClick.");
                    [NewNode(id_vec.clone())]
                })
            >{"+"}</button>
        }
    }

    fn erase_button_view(&self, id: &EntryId) -> Html {
        use Msg::*;
        let id = id.clone();
        html! {
            <button class="del-button"
                title="Erase node."
                onclick=self.link.callback(move |_| {
                    LOG!("OnClick.");
                    [EraseNode(id)]
                })
            >{" - "}</button>
        }
    }

    fn clearall_button_view(&self) -> Html {
        use Msg::*;
        html! {
            <button class="clear-button"
                title="Clear cube."
                ondblclick=self.link.callback(move |_| {
                    LOG!("OnDoubleClick.");
                    [ClearCube]
                })
            >{"Clear"}</button>
        }
    }
}