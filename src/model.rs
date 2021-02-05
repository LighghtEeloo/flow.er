#[allow(dead_code)]
#[allow(unused_assignments)]
#[allow(unused)]
mod editor;

use crate::util::*;
use crate::yew_util::*;
use crate::cube::prelude::*;


const KEY: &str = "yew.life.tracer.self";

#[derive(Debug)]
pub enum Msg {
    UpdateBuffer(String),
    NewCube,
    WriteCubeName,
    NewNode(Vec<EntryId>),
    WriteFace(EntryId),
    Focus,
    _Idle,
    _Debug(String)
}

pub struct Model {
    cube: Cube,
    buffer_str: String,
    refs: HashMap<EntryId, NodeRef>,
    focus_id: Option<EntryId>,
    storage: StorageService,
    link: ComponentLink<Self>,
}

impl Component for Model {
    // Note: MsgStack.
    type Message = Vec<Msg>;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let storage = StorageService::new(Area::Local).expect("storage was disabled by the user");
        let cube: Cube = {
            if let Json(Ok(restored_model)) = storage.restore(KEY) {
                restored_model
            } else {
                Cube::new()
            }
        };

        // Test..
        // let cube = Cube::new();
        // cube.name = format!("Ehaema!");
        // // cube.locked = true;
        // let mut entry = Entry::new();
        // entry.set_face(format!("???"));
        // cube.entries.insert(entry.id(), entry.clone());
        // cube.relation = RelationModel::Linear(vec!(entry.id()));
        LOG!("Loaded: {:#?}", cube);
        
        let id_iter = cube.entries.keys().map(|x| (x.clone(),NodeRef::default()));
        let refs: HashMap<EntryId, NodeRef> = HashMap::from_iter(id_iter);
        Self {
            cube,
            buffer_str: String::new(),
            refs,
            focus_id: None,
            storage,
            link,
        }
    }

    fn update(&mut self, messages: Self::Message) -> ShouldRender {
        // Test..
        LOG!("Updating with: {:?}", messages);
        use Msg::*;
        for msg in messages {
            match msg {
                UpdateBuffer(val) => {
                    self.buffer_str = val;
                }
                NewCube => {
                    // Todo: Add new cube.
                    LOG!("NewCube");
                    self.cube.name = mem::take(&mut self.buffer_str);
                }
                WriteCubeName => {
                    self.cube.name = mem::take(&mut self.buffer_str);
                }
                NewNode(vec) => {
                    if vec.len() == 0 {
                        let new_id = self.cube.grow();
                        self.cube.tiptoe(new_id);
                        self.refs.insert(new_id, NodeRef::default());
                        self.focus_id = Some(new_id);
                        self.link.callback(move |_: Msg| [Focus] ).emit(_Idle);
                    } else {
                        // Todo: change the semantics.
                        for root_id in vec {
                            let new_id = self.cube.grow();
                            self.cube.chain(new_id, root_id);
                            self.refs.insert(new_id, NodeRef::default());
                            self.focus_id = Some(new_id);
                            self.link.callback(move |_: Msg| [Focus] ).emit(_Idle);
                        }
                    }
                }
                WriteFace(id) => {
                    LOG!("Buffer: {}", self.buffer_str);
                    let x: &mut Entry = self.cube.entries.get_mut(&id).unwrap();
                    x.set_face(mem::take(&mut self.buffer_str));
                }
                Focus => {
                    LOG!("Focus.");
                    if let Some(focus_id) = self.focus_id {
                        if let Some(input) = self.refs.get(&focus_id).unwrap().cast::<InputElement>() {
                            input.focus().unwrap();
                        }
                    }
                }
                // _Debug(debug) => {
                //     self.storage.store(KEY, debug);
                // }
                _ => {}
            }
        }
        // Note: Only self.cube is saved.
        self.storage.store(KEY, Json(&self.cube));
        LOG!("Just dumped.");
        LOG!("With {}: {:#?}", KEY, &self.cube);

        true
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let view = html! {
            <div class="app-wrapper">
                <div class="frame" id="left-sidebar">
                    { Model::sidebar_tabs() }
                </div>
                <div class="frame" id="main-editor">
                    { self.main_editor() }
                </div>
                <div class="frame" id="status-bar">
                    <p>{"Lorem ipsum dolor sit amet consectetur, adipisicing elit. Earum et voluptates atque, neque sint iste possimus at rerum accusantium quidem. Quia laborum vitae ex sed alias quisquam quibusdam at cupiditate."}</p>
                </div>
                <footer class="info" style="display: none">
                    <p>{ "Double-click to edit a todo" }</p>
                    <p>{ "Written by " }<a href="https://github.com/LighghtEeloo/" target="_blank">{ "LighghtEeloo" }</a></p>
                    <p>{ "Inspired by " }<a href="https://github.com/DenisKolodin/" target="_blank">{ "Denis Kolodin" }</a></p>
                </footer>
            </div>
        };
        view
    }
}



impl Model {
    fn sidebar_tabs() -> Html {
        let tab_meta: Vec<(&str, &str, bool)> = vec! {
            ("static/icons/hexagons.svg", "Workspace", false),
            ("static/icons/branch.svg", "Projects", false),
            ("static/icons/history.svg", "History", false),
            ("static/icons/settings.svg", "Settings", true),
        };
        let sidebar_tabs: Html = 
            tab_meta.iter().map(
                |(src, describe, bottom)| {
                    html! {
                        <li class={if !bottom {"tab"} else {"tab tab-bottom"}}>
                            <div class="tab-content">
                                <img src={&*src} alt={&*describe}/>
                                <span class="tooltip">{&*describe}</span>
                            </div>
                        </li>
                    }
                }
            ).collect();
        html! {
            { sidebar_tabs }
        }
    }

    fn main_editor(&self) -> Html {
        let view_new = html! {
            <div class="cube-new">
            { self.cube_new_input_view() }
            </div>
        };
        let view_main = self.cube_view();

        // Test: cube - new?
        if self.cube.empty() && self.cube.name.is_empty() { view_new } else { view_main }
    }

}
