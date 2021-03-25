use super::{Cube, CubeType};
use crate::EntityId;

pub struct FlowView {
    pub obj: EntityId,
    pub current: Option<EntityId>,
}

impl From<FlowView> for Cube {
    fn from(flow: FlowView) -> Self {
        Self {
            cube_type: CubeType::FlowView,
            obj: Some(flow.obj),
            current_id: flow.current,
            ..Self::default()
        }
    }
}
