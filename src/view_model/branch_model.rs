use crate::util::*;
use crate::yew_util::*;
use crate::stockpile::prelude::*;
use crate::view_model::{Model, Message, GlobalMessage};

#[derive(Debug, Clone)]
pub struct BranchModel {
    // src_view. true if src-code-view, false if cube view.
    pub src_view: bool,
    // erase_lock. true if locked, false if to-erase.
    pub erase_lock: bool,
    pub branch: Branch,
    pub buffer_str: String,
    pub refs: HashMap<CubeId, NodeRef>,
    pub link: ComponentLink<Model>
}

#[derive(Debug, Clone)]
pub enum BranchMessage {
    UpdateBuffer(String),

    ClearBranch,

    NewCube(Option<CubeId>),
    WriteName(CubeId),
    EraseCube(CubeId),

    SetFocusId(Option<CubeId>),
    Wander(Direction, bool),
    Focus,

    /// None if bare toggle; Some if force turn on / off.
    SrcViewToggle(Option<bool>),
    // Todo: _LogBranch -> Refresh.
    _LogBranch,

}

pub type BranchMessages = Vec<BranchMessage>;

impl BranchModel {
    pub fn branch_create(branch: &Branch, link: &ComponentLink<Model>) -> BranchModel {
        let cube_id_iter = branch.cubes.keys().map(|x| (x.clone(),NodeRef::default()));
        let cube_ref = HashMap::from_iter(cube_id_iter);
        BranchModel {
            src_view: false,
            erase_lock: true,
            branch: branch.clone(),
            buffer_str: String::new(),
            refs: cube_ref,
            link: link.clone()
        }
    }
    pub fn revisit(&mut self, msg: Message) {
        self.link.callback(move |_: ()| msg.clone() ).emit(());
    }
    pub fn branch_update(&mut self, messages: BranchMessages) -> ShouldRender {
        use BranchMessage::*;
        if messages.is_empty() { return true; }
        // Test..
        LOG!("|--- buffer: {:?}", self.buffer_str);
        let old_erase_lock = self.erase_lock;
        for message in messages {
            match message {
                UpdateBuffer(val) => {
                    self.buffer_str = val;
                }
                ClearBranch => {
                    self.branch.flow.clear();
                    self.branch.cubes.clear();
                    self.revisit(Globaly![GlobalMessage::ClearEditorInfo])
                }

                NewCube(des) => {
                    let id = self.branch.grow();
                    self.branch.chain(id, des);
                    self.refs.insert(id, NodeRef::default());
                }
                WriteName(id) => {
                    self.branch.get_mut(id).name = mem::take(&mut self.buffer_str);
                }
                EraseCube(id) => {
                    if self.erase_lock {
                        self.erase_lock = false;
                    } else {
                        self.branch.erase(id);
                        self.revisit( Branchy![Focus] );
                        self.erase_lock = true;
                    }
                }
                SetFocusId(id) => {
                    match id {
                        Some(id) => self.branch.flow.focus(id),
                        None => self.branch.flow.pos = None
                    }
                }
                Wander(dir, fixed) => {
                    self.branch.flow.wander(dir, fixed);
                    self.revisit( Branchy![Focus] );
                }
                Focus => {
                    let id = self.branch.flow.current()
                        .or(self.branch.flow.root)
                        .or(self.branch.flow.orphans.get(0).cloned());
                    // Note: if Some, focus(); None then do nothing.
                    if let Some(id) = id {
                        if self.refs.get(&id).is_none() {
                            self.refs.insert(id, NodeRef::default());
                        }
                        let ref_obj = self.refs.get(&id).unwrap();
                        if let Some(input) = ref_obj.cast::<InputElement>() {
                            input.focus().unwrap();
                        }
                    }
                }
                SrcViewToggle(x) => {
                    let src_view = match x {
                        None => !self.src_view,
                        Some(x) => x
                    };
                    let writing = 
                        if src_view {
                            self.buffer_str = export_json(&self.branch);
                            true 
                        } else { 
                            match from_json_str(&self.buffer_str) {
                                Ok(branch) => { 
                                    self.branch = branch;
                                    self.refs.clear();
                                    self.refs.extend(
                                        self.branch.cubes.keys().map(|&k| (k, NodeRef::default()) )
                                    );
                                    true 
                                }
                                _ => false
                            }
                        };
                    if writing { self.src_view = src_view }
                }
                _LogBranch => {
                    LOG!("{:#?}", &self.branch);
                },
            }
        }

        // Note: Restore lock.
        if old_erase_lock == false {
            self.erase_lock = true;
        }


        true
    }
}
