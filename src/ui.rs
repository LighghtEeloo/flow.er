#[allow(dead_code)]
#[allow(unused_assignments)]
#[allow(unused)]
mod view;
mod cube_editor;
mod cube_update;

use crate::util::*;
use crate::yew_util::*;
use crate::stockpile::prelude::*;

pub use cube_update::{CubeMessage, CubeMessages};

const KEY: &str = "yew.life.tracer.self";

#[derive(Debug, Clone)]
pub enum Message {
    Cube(CubeMessages),
    // Todo: Branch.
    Branch,
    _Idle
}

pub struct Model {
    // src_view. true if src-code-view, false if cube view.
    src_view: bool,
    // erase_lock. true if locked, false if to-erase.
    erase_lock: bool,
    cube: Cube,
    buffer_str: String,
    refs: HashMap<EntryId, NodeRef>,
    ref_cube_name: NodeRef,
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

        // Debug..
        LOG!("Loaded: {:#?}", cube);
        
        let id_iter = cube.entries.keys().map(|x| (x.clone(),NodeRef::default()));
        let refs: HashMap<EntryId, NodeRef> = HashMap::from_iter(id_iter);
        Self {
            src_view: false,
            erase_lock: true,
            cube,
            buffer_str: String::new(),
            refs,
            ref_cube_name: NodeRef::default(),
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
        self.main_view()
    }
}

impl Model {
    pub fn revisit(&mut self, msg: Message) {
        self.link.callback(move |_: ()| msg.clone() ).emit(());
    }
}

