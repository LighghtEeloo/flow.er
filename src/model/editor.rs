use crate::model::*;
use crate::yew_util::*;

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
        use RelationModel::*;
        let relation = &self.cube.relation;
        match relation {
            Linear(linear) => {
                let vec = linear.model;
                html! {
                    <div class="cube">
                        { self.cube_input_view() }
                        { self.add_button_view(vec![]) }
                        { for vec.iter().map(|id| self.node_view(id)) }
                        { self.clearall_button_view() }
                    </div>
                }
            }
            _ => html! {}
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
                { self.node_input_view(&id) }
                { self.add_button_view(vec![id]) }
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
                        "ArrowUp" => vec![], 
                        "ArrowDown" => vec![], 
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
                title="New node."
                onclick=self.link.callback(move |_| {
                    LOG!("OnClick.");
                    [NewNode(id_vec.clone())]
                })
            >{"+"}</button>
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