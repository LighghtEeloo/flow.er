#[allow(dead_code)]
#[allow(unused_assignments)]
#[allow(unused)]
mod view;
mod cube_editor;
mod cube;

use crate::util::*;
use crate::yew_util::*;
use crate::stockpile::prelude::*;

pub use cube::{CubeMessage, CubeMessages};

const KEY: &str = "yew.life.tracer.self";

#[derive(Debug)]
pub enum Message {
    Cube(cube::CubeMessages),
    // Todo: Branch.
    Branch,
    _Idle
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
    type Message = Message;
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
        use Message::*;
        match messages {
            Cube(msg) => self.cube_update(msg),
            // Branch => true,
            _ => true
        }
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


