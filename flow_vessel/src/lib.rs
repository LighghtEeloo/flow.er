mod identity;
mod entity;
mod time;
mod tag;

use flow_arena::{Flow, FlowArena, Node};
use identity::{EntityId, EntityIdFactory};
use entity::Entity;
use std::fmt::Debug;

pub struct Vessel {
    flow_arena: FlowArena<EntityId, Entity>,

    id_factory: EntityIdFactory
}

impl Vessel {
    pub fn new() -> Self {
        Self {
            flow_arena: FlowArena::new(),
            id_factory: EntityIdFactory::default(),
        }
    }
}

impl Debug for Vessel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Vessel")
         .field("root", &self.flow_arena.root)
         .field("node_map", &self.flow_arena.node_map)
         .field("id_factory", &self.id_factory)
         .finish()
    }
}

impl Vessel {
    pub fn entity_grow(&mut self) -> EntityId {
        let entity = Entity::new_time(&self.id_factory);
        self.entity_insert(entity)
    }
    pub fn entity_insert(&mut self, entity: Entity) -> EntityId {
        let id = entity.id().clone();
        self.flow_arena.grow(Node::from_id(id.clone(), entity)).ok();
        id
    }
    pub fn entity_get(&mut self, id: &EntityId) -> Option<&mut Entity> {
        self.flow_arena.node_map.get_mut(id).map(|x| &mut x.entity)
    }
    // pub fn entity_ensure(&mut self, id: &EntityId) -> &mut Entity {
    //     &mut self.flow_arena.node_map
    //     .entry(id.clone())
    //     .or_insert_with(|| {
    //         let entity = Entity::new_time(&self.id_factory);
    //         Node::from_id(entity.id().clone(), entity)
    //     }).entity
    //     // match self.flow_arena.node_map.get_mut(id).map(|x| &mut x.entity) {
    //     //     Some(x) => x,
    //     //     None => {
    //     //         let entity = Entity::new_time(&self.id_factory);
    //     //         self.flow_arena.node_map.insert(entity.id().clone(), Node::from_id(entity.id().clone().clone(), entity.clone()));
    //     //         &mut self.flow_arena.node_map.get_mut(&entity.id().clone()).unwrap().entity
    //     //     }
    //     // }
    // }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn vessel() {
        let vessel: Vessel = Vessel::new();
        println!("{:#?}", vessel);
    }
    #[test]
    fn entity() {
        let mut vessel = Vessel::new();
        let id = vessel.entity_grow();
        println!("{:#?}", vessel);
        println!("{:?}", vessel.entity_get(&id));
    }
}
