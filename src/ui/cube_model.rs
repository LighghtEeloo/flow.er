use crate::util::*;
use crate::yew_util::*;
use crate::stockpile::prelude::*;
use crate::ui::{Model, Message};


#[derive(Debug, Clone)]
pub struct CubeModel {
    // src_view. true if src-code-view, false if cube view.
    pub src_view: bool,
    // erase_lock. true if locked, false if to-erase.
    pub erase_lock: bool,
    pub cube: Cube,
    pub buffer_str: String,
    pub refs: HashMap<EntryId, NodeRef>,
    pub ref_cube_name: NodeRef,
    pub link: ComponentLink<Model>
}

impl CubeModel {
    pub fn revisit(&mut self, msg: Message) {
        self.link.callback(move |_: ()| msg.clone() ).emit(());
    }
}
