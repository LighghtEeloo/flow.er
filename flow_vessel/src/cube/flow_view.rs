use std::collections::HashSet;
use crate::{EntityId, Vessel};
use super::{Cube, CubeType, CubeMember};

pub struct FlowView {
    pub obj: EntityId,
    pub current: Option<EntityId>,
}

impl Into<Cube> for FlowView {
    fn into(self) -> Cube {
        Cube {
            cube_type: CubeType::FlowView,
            obj: Some(self.obj),
            current: self.current,
            ..Cube::default()
        }
    }
}

impl From<Cube> for FlowView {
    fn from(cube: Cube) -> Self {
        Self {
            obj: cube.obj.unwrap_or_default(),
            current: cube.current,
        }
    }
}

impl CubeMember for FlowView {
    fn member_traverse(&self, vessel: &Vessel) -> HashSet<EntityId> {
        vessel.entity_offspring(&self.obj)
    }
}
