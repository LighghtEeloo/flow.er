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
    pub fn branch_update(&mut self, messages: BranchMessages) -> ShouldRender {
        match messages {
            _ => ()
        }
        true
    }
}
