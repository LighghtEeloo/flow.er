use crate::{EntityId};
use super::{Cube, CubeType, CubeMember};

pub struct Inkblot {
    pub obj: EntityId
}

impl Into<Cube> for Inkblot {
    fn into(self) -> Cube {
        Cube {
            cube_type: CubeType::Inkblot,
            obj: Some(self.obj),
            ..Cube::default()
        }
    }
}

impl From<Cube> for Inkblot {
    fn from(cube: Cube) -> Self {
        Self {
            obj: cube.obj.unwrap_or_default()
        }
    }
}

impl CubeMember for Inkblot {}
