use std::fmt::Debug;
use serde::{Serialize, Deserialize};

use super::{Entity, EntityId, EntityIdFactory, Node, Flow, FlowArena, Router, Glass};

#[derive(Default, Clone, Serialize, Deserialize)]
pub struct Vessel {
    flow_arena: FlowArena<EntityId, Entity>,
    id_factory: EntityIdFactory,
    pub glass: Glass,
    pub router: Router,
}

impl Vessel {
    pub fn new() -> Self {
        Self {
            flow_arena: FlowArena::new(),
            id_factory: EntityIdFactory::default(),
            glass: Glass::default(),
            router: Router::default()
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
    pub fn entity_get(&mut self, id: &EntityId) -> Option<&Entity> {
        self.flow_arena.node_map.get(id).map(|x| &x.entity)
    }
    pub fn entity_get_mut(&mut self, id: &EntityId) -> Option<&mut Entity> {
        self.flow_arena.node_map.get_mut(id).map(|x| &mut x.entity)
    }
    pub fn entity_list(&self, id: &EntityId) -> Vec<&Entity> {
        let vec = self.flow_arena.node_map.get(id).map(|x| x.children.clone()).unwrap_or_default();
        
        vec.into_iter().filter_map(|id| self.flow_arena.node_map.get(&id)).map(|x| &x.entity).collect()
    }
    pub fn entity_ensure(&mut self, id: &EntityId) -> &mut Entity {
        if !self.flow_arena.node_map.contains_key(id) {
            self.entity_insert(Entity::new_id(id));
        }
        self.entity_get_mut(id).expect("contains key")
    }
    pub fn entity_decay(&mut self, id: &EntityId) {
        self.flow_arena.decay(id).ok();
    }
}

pub type EntityNode = Node<EntityId, Entity>;

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
        println!("{:?}", vessel.entity_get_mut(&id));
    }
    #[test]
    fn entity_ensure() {
        let mut vessel = Vessel::new();
        let _id = vessel.entity_grow();
        let id1 = vessel.id_factory.incr_id();
        vessel.entity_ensure(&id1);
        println!("{:#?}", vessel);
    }
    #[test]
    fn entity_list() {
        let mut vessel = Vessel::new();
        let id = vessel.entity_grow();
        let id1 = vessel.entity_grow();
        let id2 = vessel.entity_grow();
        let id3 = vessel.entity_grow();
        let id4 = vessel.entity_grow();
        vessel.flow_arena.devote_push(&id1, &id).ok();
        vessel.flow_arena.devote_push(&id2, &id).ok();
        vessel.flow_arena.devote_push(&id3, &id).ok();
        vessel.flow_arena.devote_push(&id4, &id).ok();
        vessel.flow_arena.devote_push(&id4, &id1).ok();
        // 0_0 --> id --> [id1, id2, id3]
        //          `-------`-> id4
        vessel.entity_get_mut(&id).map(|entity| entity.face = format!("Aloha!"));
        println!("{:#?}", vessel);
        println!("{:#?}", vessel.entity_get(&id));
        println!("{:#?}", vessel.entity_list(&id));
    }
    #[test]
    fn entity_decay() {
        let mut vessel = Vessel::new();
        let id = vessel.entity_grow();
        let id1 = vessel.entity_grow();
        let id2 = vessel.entity_grow();
        let id3 = vessel.entity_grow();
        let id4 = vessel.entity_grow();
        vessel.flow_arena.devote_push(&id1, &id).ok();
        vessel.flow_arena.devote_push(&id2, &id).ok();
        vessel.flow_arena.devote_push(&id3, &id).ok();
        vessel.flow_arena.devote_push(&id4, &id).ok();
        vessel.flow_arena.devote_push(&id4, &id1).ok();
        // 0_0 --> id --> [id1, id2, id3]
        //          `-------`-> id4
        vessel.entity_get_mut(&id).map(|entity| entity.face = format!("Aloha!"));
        vessel.entity_get_mut(&id1).map(|entity| entity.face = format!("Bobi."));
        vessel.entity_decay(&id);
        println!("{:#?}", vessel);
    }
    #[test]
    fn serde() {
        let mut vessel = Vessel::new();
        let id = vessel.entity_grow();
        let id1 = vessel.entity_grow();
        let id2 = vessel.entity_grow();
        let id3 = vessel.entity_grow();
        let id4 = vessel.entity_grow();
        vessel.flow_arena.devote_push(&id1, &id).ok();
        vessel.flow_arena.devote_push(&id2, &id).ok();
        vessel.flow_arena.devote_push(&id3, &id).ok();
        vessel.flow_arena.devote_push(&id4, &id).ok();
        vessel.flow_arena.devote_push(&id4, &id1).ok();
        // 0_0 --> id --> [id1, id2, id3]
        //          `-------`-> id4
        vessel.entity_get_mut(&id).map(|entity| entity.face = format!("Aloha!"));
        vessel.entity_get_mut(&id1).map(|entity| entity.face = format!("Bobi."));
        let str = serde_json::to_string(&vessel).expect("failed to serialize vessel");
        println!("Serialize: {}", str);
        let vessel: Vessel = serde_json::from_str(&str).expect("failed to deserialize");
        println!("{:#?}", vessel);
    }
}
