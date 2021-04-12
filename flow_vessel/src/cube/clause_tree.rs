use std::collections::{HashMap, HashSet};
use crate::{EntityId, EntityNode, Vessel};
use super::{Cube, CubeType, CubeMember};

pub struct ClauseTreeCube {
    pub obj: EntityId,
    pub current: Option<usize>,
}

impl Into<Cube> for ClauseTreeCube {
    fn into(self) -> Cube {
        Cube {
            cube_type: CubeType::ClauseTree,
            obj: Some(self.obj),
            current_idx: self.current,
            ..Cube::default()
        }
    }
}

impl From<Cube> for ClauseTreeCube {
    fn from(cube: Cube) -> Self {
        Self {
            obj: cube.obj.unwrap_or_default(),
            current: cube.current_idx,
        }
    }
}

impl CubeMember for ClauseTreeCube {
    fn member_traverse(&self, vessel: &Vessel) -> HashSet<EntityId> {
        vessel.entity_ownership(&self.obj)
    }
}

#[derive(Default, Clone)]
pub struct ClauseTreeCore {
    cube: Cube,
    node_map: HashMap<EntityId, EntityNode>
}

impl ClauseTreeCore {
    pub fn new(cube: Cube, vessel: &Vessel) -> Self {
        let set = 
            ClauseTreeCube::from(cube.clone()).member_traverse(vessel);
        let node_map = 
            set.into_iter()
            .filter_map(|x| vessel.node(&x).cloned())
            .map(|x| (x.id().clone(), x))
            .collect();
        Self {
            cube,
            node_map
        }
    }
}
