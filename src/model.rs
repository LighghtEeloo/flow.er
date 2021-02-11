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
    SetFocusId(Option<EntryId>),
    Wander(Direction, bool),
    Focus,
    _Idle,
}

pub struct Model {
    cube: Cube,
    buffer_str: String,
    refs: HashMap<EntryId, NodeRef>,
    ref_name: NodeRef,
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

        LOG!("Loaded: {:#?}", cube);
        
        let id_iter = cube.entries.keys().map(|x| (x.clone(),NodeRef::default()));
        let refs: HashMap<EntryId, NodeRef> = HashMap::from_iter(id_iter);
        Self {
            cube,
            buffer_str: String::new(),
            refs,
            ref_name: NodeRef::default(),
            storage,
            link,
        }
    }

    fn update(&mut self, messages: Self::Message) -> ShouldRender {
        // Test..
        if messages.is_empty() { return true; }
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
                        self.link.callback(move |_: Msg| [Focus] ).emit(_Idle);
                    } else {
                        // Todo: change the semantics.
                        for root_id in vec {
                            let new_id = self.cube.grow();
                            self.cube.chain(new_id, root_id);
                            self.refs.insert(new_id, NodeRef::default());
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
                    self.link.callback(move |_: Msg| [Focus] ).emit(_Idle);
                }
                SetFocusId(id) => {
                    match id {
                        Some(id) => self.cube.relation.focus(id),
                        None => self.cube.relation.pos = None
                    }
                }
                Wander(dir, fixed) => {
                    // Todo: register shift key.
                    self.cube.relation.wander(dir, fixed);
                    self.link.callback(move |_: Msg| [Focus] ).emit(_Idle);
                }
                Focus => {
                    let id = self.cube.relation.current();
                    // Debug..
                    LOG!("Focusing: {:?}", id);
                    match id {
                        Some(id) => {
                            if let Some(input) = self.refs.get(&id).unwrap().cast::<InputElement>() {
                                input.focus().unwrap();
                            }
                        }
                        None => {
                            if let Some(input) = self.ref_name.cast::<InputElement>() {
                                input.focus().unwrap();
                            }
                        }
                    }
                }
                _Idle => {}
            }
        }
        // Note: Only self.cube is saved.
        self.storage.store(KEY, Json(&self.cube));
        // Debug..
        LOG!("Dumped {}: {:#?}", KEY, &self.cube);

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


