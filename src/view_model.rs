#[allow(dead_code)]
#[allow(unused_assignments)]
#[allow(unused)]
mod main_view;
mod cube_model;
mod cube_editor;
mod branch_model;
mod branch_editor;
mod history_model;
mod history_editor;

use crate::util::*;
use crate::yew_util::*;
use crate::stockpile::prelude::*;

pub use cube_model::{CubeModel, CubeMessage, CubeMessages};
pub use branch_model::{BranchModel, BranchMessage, BranchMessages};
pub use history_model::{HistoryModel, HistoryMessage, HistoryMessages};

const KEY: &str = "yew.life.tracer.self";

#[derive(Debug, Clone, Copy, PartialEq)]
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
            History => Historyly![HistoryMessage::_LogHistory],
            // Todo: Settings.
            _ => Message::_Idle
        }
    }
}

pub struct Model {
    router: Router,
    cube_model: CubeModel,
    branch_model: BranchModel,
    history_model: HistoryModel,
    stockpile: Stockpile,
    storage: StorageService,
    link: ComponentLink<Self>,
}


#[derive(Debug, Clone)]
pub enum Message {
    Cube(CubeMessages),
    Branch(BranchMessages),
    History(HistoryMessages),
    // Todo: Settings.
    Settings,
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
            let stockpile = if let Json(Ok(restored_model)) = storage.restore(KEY) {
                restored_model
            } else {
                Stockpile::new()
            };
            // Debug..
            // let mut stockpile = Stockpile::new();
            // stockpile.editor_info = EditorInfo::new_some(CubeId::new());
            stockpile
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

        
        // Todo: Use RefCell.
        let history_stockpile: &Stockpile = &stockpile;
        let history_model = HistoryModel::history_create(history_stockpile, &link);

        Self {
            router: Router::Cube,
            cube_model,
            branch_model,
            history_model,
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
            History(msg) => {
                self.stockpile_read();
                let res = self.history_model.history_update(msg);
                self.stockpile_write();
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
        // Note: editor_info > cube_model.cube.id(), see _impl.
        self.cube_model.cube = Model::cube_read_impl(&mut self.stockpile).clone();
    }
    pub fn cube_write(&mut self) {
        // Note: update editor_info with cube_model.cube.id()
        let cube_id = self.cube_model.cube.id();
        self.stockpile.branch.cubes
            .insert(cube_id, self.cube_model.cube.clone());
            // .expect("failed to write cube");
        self.stockpile.editor_info = EditorInfo::new_some(cube_id);
    }
    pub fn branch_read(&mut self) {
        self.branch_model.branch = self.stockpile.branch.clone();
    }
    pub fn branch_write(&mut self) {
        self.stockpile.branch = self.branch_model.branch.clone();
    }
    pub fn stockpile_read(&mut self) {
        self.history_model.stockpile = self.stockpile.clone();
    }
    pub fn stockpile_write(&mut self) {
        self.stockpile = self.history_model.stockpile.clone();
    }

    // impl

    fn cube_read_impl(stockpile: &mut Stockpile) -> &Cube {
        match None 
        .or(stockpile.editor_info.cube_id)
        .or(stockpile.branch.flow.current())
        .or(stockpile.branch.flow.root)
        .or(stockpile.branch.flow.orphans.get(0).cloned()) {
            Some(id) => {
                stockpile.branch.get_update(id)
            }
            None => {
                let cube_id = stockpile.branch.grow();
                stockpile.branch.tiptoe(cube_id);
                stockpile.branch.get_update(cube_id)
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

// global_update

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
                            self.revisit(Cubey![SrcViewToggle(None)]);
                        }
                        Router::Branch => {
                            use BranchMessage::*;
                            self.revisit(Branchy![SrcViewToggle(None)]);
                        }
                        Router::History => {
                            use HistoryMessage::*;
                            self.revisit(Historyly![SrcViewToggle(None)]);
                        }
                        // Todo: other src-view toggles.
                        _ => ()
                    }
                    self.src_all_close(self.router);
                }
                SwitchRouter(router) => {
                    self.router = router;
                    self.revisit(router.refresh_message())
                }
                ClearEditorInfo => {
                    self.stockpile.editor_info = EditorInfo::new_none();
                }
            }
        }
        true
    }
    fn src_all_close(&mut self, keeping_router: Router) {
        if keeping_router != Router::Cube {
            self.revisit(Cubey![CubeMessage::SrcViewToggle(Some(false))]);
        }
        if keeping_router != Router::Branch {
            self.revisit(Branchy![BranchMessage::SrcViewToggle(Some(false))]);
        }
        if keeping_router != Router::History {
            self.revisit(Historyly![HistoryMessage::SrcViewToggle(Some(false))]);
        }

    }
}

