use crate::EntityId;
use super::{Cube, CubeType};

pub struct ClauseTree {
    pub obj: EntityId,
    pub current: Option<usize>
}

impl From<ClauseTree> for Cube {
    fn from(clause_tree: ClauseTree) -> Self {
        Self {
            cube_type: CubeType::ClauseTree,
            obj: Some(clause_tree.obj),
            current_idx: clause_tree.current,
            ..Self::default()
        }
    }
}
