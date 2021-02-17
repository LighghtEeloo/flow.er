#[allow(dead_code)]
#[allow(unused_assignments)]
#[allow(unused)]
mod main_view;
mod cube_model;
mod cube_editor;
mod branch_model;
mod branch_editor;

use crate::util::*;
use crate::yew_util::*;
use crate::stockpile::prelude::*;

pub use cube_model::{CubeModel, CubeMessage, CubeMessages};
pub use branch_model::{BranchModel, BranchMessage, BranchMessages};

const KEY: &str = "yew.life.tracer.self";

#[derive(Debug, Clone, Copy)]
pub enum Router {
    Cube,
    Branch,
    History,
    Settings,
}

impl Router {
    fn refresh_message(&self) -> Message {
        // Note: This refreshes and syncs the specific model.
        use Router::*;
        match self {
            Cube => Cubey![CubeMessage::_LogCube],
            Branch => Branchy![BranchMessage::_LogBranch],
            // Todo: History
            _ => Message::_Idle
        }
    }
}

pub struct Model {
    router: Router,
    cube_model: CubeModel,
    branch_model: BranchModel,
    stockpile: Stockpile,
    storage: StorageService,
    link: ComponentLink<Self>,
}


#[derive(Debug, Clone)]
pub enum Message {
    Cube(CubeMessages),
    Branch(BranchMessages),
    // Todo: History.
    History,
    Global(GlobalMessages),
    _Debug(String),
    _Idle
}

#[derive(Debug, Clone)]
pub enum GlobalMessage {
    SrcHit,
    SwitchRouter(Router),
    ClearEditorInfo,
}
pub type GlobalMessages = Vec<GlobalMessage>;

impl Component for Model {
    // Note: MsgStack.
    type Message = Message;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let storage = StorageService::new(Area::Local).expect("storage was disabled by the user");
        let mut stockpile: Stockpile = {
            if let Json(Ok(restored_model)) = storage.restore(KEY) {
                restored_model
            } else {
                Stockpile::new()
            }
        };
        // Note: clean only on startup.
        stockpile.branch.clean();
        // Test..
        LOG!("Loaded & Cleaned: {:#?}", stockpile);
        
        // Todo: Use RefCell.
        let cube: &Cube = Model::cube_read_impl(&mut stockpile);
        let cube_model = CubeModel::cube_create(cube, &link);

        
        // Todo: Use RefCell.
        let branch: &Branch = &stockpile.branch;
        let branch_model = BranchModel::branch_create(branch, &link);

        Self {
            router: Router::Cube,
            cube_model,
            branch_model,
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
                self.cube_read();
                let res = self.cube_model.cube_update(msg);
                self.cube_write();
                res
            }
            Branch(msg) => {
                self.branch_read();
                let res = self.branch_model.branch_update(msg);
                self.branch_write();
                res
            }
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
    pub fn cube_read(&mut self) {
        // Note: editor_info > cube_model.cube.id()
        self.cube_model.cube = Model::cube_read_impl(&mut self.stockpile).clone();
    }
    pub fn cube_write(&mut self) {
        // Note: update editor_info with cube_model.cube.id()
        let cube_id = self.cube_model.cube.id();
        self.stockpile.branch.cubes
            .insert(cube_id, self.cube_model.cube.clone());
            // .expect("failed to write cube");
        self.stockpile.editor_info = Some( EditorInfo {
            cube_id: Some(cube_id),
            entry_id: self.cube_model.cube.relation.current()
        });
    }
    pub fn branch_read(&mut self) {
        self.branch_model.branch = self.stockpile.branch.clone();
    }
    pub fn branch_write(&mut self) {
        self.stockpile.branch = self.branch_model.branch.clone();
    }

    // impl

    fn cube_read_impl(stockpile: &mut Stockpile) -> &Cube {
        match stockpile.editor_info.clone().map(|x| x.cube_id).flatten()
        .or(stockpile.branch.flow.current())
        .or(stockpile.branch.flow.root)
        .or(stockpile.branch.flow.orphans.get(0).cloned()) {
            Some(id) => {
                stockpile.branch.get(id)
            }
            None => {
                let cube_id = stockpile.branch.grow();
                stockpile.branch.tiptoe(cube_id);
                stockpile.branch.get(cube_id)
            }
        }
    }

    pub fn src_view_status(&self) -> bool {
        match self.router {
            Router::Cube => self.cube_model.src_view,
            Router::Branch => self.branch_model.src_view,
            _ => false
        }
    }
}

impl Model {
    pub fn revisit(&mut self, msg: Message) {
        self.link.callback(move |_: ()| msg.clone() ).emit(());
    }
    pub fn global_update(&mut self, msgs: GlobalMessages) -> ShouldRender {
        use GlobalMessage::*;
        for msg in msgs {
            match msg {
                SrcHit => {
                    match self.router {
                        Router::Cube => {
                            use CubeMessage::*;
                            self.revisit(Cubey![SrcViewToggle(None)])
                        }
                        Router::Branch => {
                            use BranchMessage::*;
                            self.revisit(Branchy![SrcViewToggle(None)])
                        }
                        // Todo: other src-view toggles.
                        _ => ()
                    }
                }
                SwitchRouter(router) => {
                    self.router = router;
                    self.revisit(router.refresh_message())
                }
                ClearEditorInfo => {
                    self.stockpile.editor_info = None;
                }
            }
        }
        true
    }
}

