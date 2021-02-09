#[allow(dead_code)]
#[allow(unused_assignments)]
#[allow(unused)]
mod view;
mod editor;

use crate::util::*;
use crate::yew_util::*;
use crate::cube::prelude::*;


const KEY: &str = "yew.life.tracer.self";

#[derive(Debug)]
pub enum Msg {
    UpdateBuffer(String),
    NewCube,
    ClearCube,
    WriteCubeName,
    NewNode(Vec<EntryId>),
    WriteFace(EntryId),
    WriteProcess(EntryId),
    EraseNode(EntryId),
    Focus,
    _Idle,
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
        // Todo: Register ctrl.
        let storage = StorageService::new(Area::Local).expect("storage was disabled by the user");
        let cube: Cube = {
            if let Json(Ok(restored_model)) = storage.restore(KEY) {
                restored_model
            } else {
                Cube::new()
            }
        };

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
        LOG!("Updating with: {:?}\nand buffer: {:?}", messages, self.buffer_str);
        use Msg::*;
        for msg in messages {
            match msg {
                UpdateBuffer(val) => {
                    self.buffer_str = val;
                }
                NewCube => {
                    // Todo: Add new cube.
                    self.cube.name = mem::take(&mut self.buffer_str);
                }
                ClearCube => {
                    self.cube.relation.clear();
                    self.cube.entries.clear();
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
                    let x: &mut Entry = self.cube.entries.get_mut(&id).unwrap();
                    x.set_face(mem::take(&mut self.buffer_str));
                }
                WriteProcess(id) => {
                    let x: &mut Entry = self.cube.entries.get_mut(&id).unwrap();
                    x.set_process(ProcessStatus::reflect(self.buffer_str.as_str()));
                }
                EraseNode(id) => {
                    self.cube.erase(id);
                }
                Focus => {
                    let id = 
                        if let None = self.focus_id {
                            self.cube.relation.current()
                        } else {
                            self.focus_id
                        };
                    match id {
                        Some(id) => {
                            if let Some(input) = self.refs.get(&id).unwrap().cast::<InputElement>() {
                                input.focus().unwrap();
                            }
                        }
                        None => ()
                    }
                }
                _Idle => {}
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
                    <p>{"Lorem ipsum dolor sit amet consectetur, adipisicing elit. Earum et voluptates atque, neque sint iste possimus at rerum accusantium quidem."}</p>
                </div>
            </div>
        };
        view
    }
}


