use std::collections::HashMap;

use flow_arena::FlowBase;

use crate::{Cube, CubeId, CubeMeta, Entity, EntityId, Vessel};

#[derive(Clone)]
pub struct NodeViewCore {
    pub cube_id: CubeId,
    pub obj: Option<EntityId>,
    pub children: Vec<EntityId>,
    pub current: Option<EntityId>,
    pub entity_map: HashMap<EntityId, (Own, Entity)>,
}

#[derive(Clone)]
pub enum Own {
    Obj,
    Yes,
    No,
}

impl NodeViewCore {
    pub fn from_router_cube(
        vessel: &Vessel,
        (_, cube_id, cube): (CubeMeta, CubeId, Cube),
    ) -> Option<Self> {
        Self::from_cube(vessel, cube_id, cube)
    }
    pub fn from_cube(
        vessel: &Vessel,
        cube_id: CubeId,
        cube: Cube,
    ) -> Option<Self> {
        let mut entity_map = HashMap::new();
        let obj = cube.obj;
        let mut children: Vec<EntityId>;
        if let Some(obj) = obj {
            entity_map.insert(
                obj,
                (Own::Obj, vessel.flow.node(&obj)?.entity.clone()),
            );
            children = vessel.flow.children(&obj);
            for c in children.iter() {
                let own = if vessel.flow.is_owned(c, &obj) {
                    Own::Yes
                } else {
                    Own::No
                };
                entity_map.insert(
                    c.clone(),
                    (own, vessel.flow.node(c)?.entity.clone()),
                );
            }
        } else {
            children = vessel.flow.orphan();
            children.sort_by_key(|i| i.clone());
            for c in children.iter() {
                let own = Own::Yes;
                entity_map.insert(
                    c.clone(),
                    (own, vessel.flow.node(c)?.entity.clone()),
                );
            }
        }
        Some(Self {
            cube_id,
            obj,
            children,
            current: None,
            entity_map,
        })
    }
}
