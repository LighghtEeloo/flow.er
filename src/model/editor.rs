use crate::model::*;
use crate::yew_util::*;
use crate::cube::prelude::*;

impl Model {
    pub fn cube_new_input_view(&self) -> Html {
        use Msg::*;
        html! {
            <div class="cube-input">
                <input
                    type="text"
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
                { self.add_button_view(vec![]) }
                { for vec.iter().map(|id| self.node_view(id)) }
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
                    placeholder="Enter new proj name."
                    value=self.cube.name
                    oninput=self.link.callback(move |e: InputData| {
                        LOG!("OnInput: {:?}", e);
                        [UpdateBuffer(e.value), WriteCubeName]
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
            vec.iter().map(
                |x| (
                    String::from(ProcessStatus::type_src(x)), 
                    String::from(ProcessStatus::type_str(x))
                ) 
            ).collect();
        let status_dropdown: Html = 
            status_meta.into_iter().map(|(src, des)| {
                html! {
                    <ul
                        title={des.clone()}
                        onclick=self.link.callback(move |_| {
                            [UpdateBuffer(des.clone()), WriteProcess(id)]
                        })
                    > 
                        <img 
                            src={src}
                        /> 
                    </ul> 
                }
            }).collect();
        html! {
            <div class="dropdown"> 
                <button 
                    class="dropbtn"
                    value=self.cube.get(id).process().type_str()
                > 
                    <img 
                        src={self.cube.get(id).process().type_src()}
                    />
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
        html! {
            <input
                type="text"
                value=self.cube.get(id).face()
                placeholder="..."
                onfocus=self.link.callback(move |_| {
                    [SetFocus(id)]
                })
                oninput=self.link.callback(move |e: InputData| {
                    LOG!("OnInput: {:?}", e);
                    [UpdateBuffer(e.value), WriteFace(id)]
                })
                ref=self.refs.get(&id).unwrap().clone()
                onkeypress=self.link.callback(move |e: KeyboardEvent| {
                    LOG!("OnKeyPress: {:?}", e);
                    match e.key().as_str() { 
                        "Enter" => vec![NewNode(vec!(id))],
                        "Backspace" => vec![], 
                        "Delete" => vec![], 
                        // Todo: Remove keyblock.
                        "ArrowUp" => vec![Wander(Direction::Up)], 
                        "ArrowDown" => vec![Wander(Direction::Down)], 
                        "ArrowLeft" => vec![], 
                        "ArrowRight" => vec![], 
                        _ => vec![] 
                    }
                })
                readonly=if self.cube.locked { true } else { false }
            />
        }
    }

    fn add_button_view(&self, id_vec: Vec<EntryId>) -> Html {
        use Msg::*;
        html! {
            <button
                // style="width: 24px; height: 24px; line-height: 24px; display: inline; margin: 0;"
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
            <button
                // style="width: 24px; height: 24px; line-height: 24px; display: inline; margin: 0;"
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
            <button
                title="Clear cube."
                ondblclick=self.link.callback(move |_| {
                    LOG!("OnDoubleClick.");
                    [ClearCube]
                })
            >{"Clear"}</button>
        }
    }
}