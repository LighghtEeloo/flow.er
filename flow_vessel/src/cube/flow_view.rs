use std::collections::HashSet;
use crate::{EntityId, Vessel};
use super::{Cube, CubeType, CubeView};

pub struct FlowView {
    pub obj: EntityId,
    pub current: Option<EntityId>,
}

impl Into<Cube> for FlowView {
    fn into(self) -> Cube {
        Cube {
            cube_type: CubeType::FlowView,
            obj: Some(self.obj),
            current_id: self.current,
            ..Cube::default()
        }
    }
}

impl From<Cube> for FlowView {
    fn from(cube: Cube) -> Self {
        Self {
            obj: cube.obj.unwrap_or_default(),
            current: cube.current_id,
        }
    }
}

impl CubeView for FlowView {
    fn member_traverse(&self, vessel: &Vessel) -> HashSet<EntityId> {
        vessel.entity_ids(&self.obj)
    }
}
