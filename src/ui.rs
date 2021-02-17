#[allow(dead_code)]
#[allow(unused_assignments)]
#[allow(unused)]
mod view;
mod cube_model;
mod cube_update;
mod cube_editor;

use crate::util::*;
use crate::yew_util::*;
use crate::stockpile::prelude::*;

pub use cube_update::{CubeMessage, CubeMessages};
pub use cube_model::CubeModel;

const KEY: &str = "yew.life.tracer.self";

#[derive(Debug, Clone)]
pub enum Router {
    Cube,
    Branch,
    History,
    Settings,
}

pub struct Model {
    router: Router,
    cube_model: CubeModel,
    stockpile: Branch,
    storage: StorageService,
    link: ComponentLink<Self>,
}


#[derive(Debug, Clone)]
pub enum Message {
    Cube(CubeMessages),
    // Todo: Branch.
    Branch,
    // Todo: History.
    History,
    Global(GlobalMessages),
    _Debug(String),
    _Idle
}

#[derive(Debug, Clone)]
pub enum GlobalMessage {
    SrcHit,
}
pub type GlobalMessages = Vec<GlobalMessage>;

impl Component for Model {
    // Note: MsgStack.
    type Message = Message;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let storage = StorageService::new(Area::Local).expect("storage was disabled by the user");
        let stockpile: Stockpile = {
            if let Json(Ok(restored_model)) = storage.restore(KEY) {
                restored_model
            } else {
                Stockpile::new()
            }
        };
        let cube: Cube =  match stockpile.flow.current() {
            Some(id) => {
                stockpile.get(id).clone()
            }
            None => Cube::new()
        };
        // Debug..
        LOG!("Loaded: {:#?}", stockpile);
        let id_iter = cube.entries.keys().map(|x| (x.clone(),NodeRef::default()));
        let refs: HashMap<EntryId, NodeRef> = HashMap::from_iter(id_iter);
        let cube_model = CubeModel {
            src_view: false,
            erase_lock: true,
            cube,
            buffer_str: String::new(),
            refs,
            ref_cube_name: NodeRef::default(),
            link: link.clone()
        };

        
        Self {
            router: Router::Cube,
            cube_model,
            stockpile,
            storage,
            link,
        }
    }

    fn update(&mut self, messages: Self::Message) -> ShouldRender {
        use Message::*;
        LOG!("Updating: {:#?}.", messages);
        let res = match messages {
            Cube(msg) => {
                let res = self.cube_model.cube_update(msg);
                self.sync();
                res
            }
            // Branch => true,
            Global(msg) => self.global_update(msg),
            _ => true
        };
        // Note: Only self.stockpile is saved.
        self.storage.store(KEY, Json(&self.stockpile));
        res
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        self.main_view()
    }
}

impl Model {
    pub fn sync(&mut self) {
        let cube_id = self.cube_model.cube.id();
        let cube: &mut Cube = self.stockpile.get_mut(cube_id);
        *cube = self.cube_model.cube.clone();
    }
    pub fn revisit(&mut self, msg: Message) {
        self.link.callback(move |_: ()| msg.clone() ).emit(());
    }
    pub fn global_update(&mut self, msgs: GlobalMessages) -> ShouldRender {
        for msg in msgs {
            match msg {
                GlobalMessage::SrcHit => {
                    match self.router {
                        // Todo..
                        _ => ()
                    }
                }
            }
        }
        true
    }
}

