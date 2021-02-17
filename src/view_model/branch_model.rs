use crate::util::*;
use crate::yew_util::*;
use crate::stockpile::prelude::*;
use crate::view_model::{Model, Message};

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

}

impl BranchMessage {
    pub fn multi(a: BranchMessages) -> Message {
        Message::Branch(a)
    }
}

pub type BranchMessages = Vec<BranchMessage>;

impl BranchModel {
    pub fn branch_update(&mut self, messages: BranchMessages) -> ShouldRender {
        match messages {
            _ => ()
        }
        true
    }
}
