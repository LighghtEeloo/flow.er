use super::{Cube, CubeType};
use crate::EntityId;

pub struct ClauseTree {
    pub obj: EntityId,
    pub current: Option<usize>,
}

impl Into<Cube> for ClauseTree {
    fn into(self) -> Cube {
        Cube {
            cube_type: CubeType::ClauseTree,
            obj: Some(self.obj),
            current_idx: self.current,
            ..Cube::default()
        }
    }
}

impl From<Cube> for ClauseTree {
    fn from(cube: Cube) -> Self {
        Self {
            obj: cube.obj.unwrap_or_default(),
            current: cube.current_idx,
        }
    }
}
