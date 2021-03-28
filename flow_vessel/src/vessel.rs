use std::fmt::Debug;
use serde::{Serialize, Deserialize};

use super::{Entity, EntityId, EntityIdFactory, Node, Flow, FlowArena, Router, Glass, Cube, Settings};

pub type EntityNode = Node<EntityId, Entity>;
pub type EntityFlow = FlowArena<EntityId, Entity>;

#[derive(Default, Clone, Serialize, Deserialize)]
pub struct Vessel {
    flow_arena: EntityFlow,
    id_factory: EntityIdFactory,
    #[serde(default)]
    pub glass: Glass,
    #[serde(default)]
    pub router: Router,
    #[serde(default)]
    pub settings: Settings
}

impl Vessel {
    pub fn new() -> Self {
        Self {
            flow_arena: FlowArena::new(),
            id_factory: EntityIdFactory::default(),
            glass: Glass::default(),
            router: Router::default(),
            settings: Settings::default()
        }
    }
}

impl Debug for Vessel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Vessel")
         .field("router", &self.router)
         .field("glass", &self.glass)
         .field("id_factory", &self.id_factory)
         .field("root", &self.flow_arena.root)
         .field("node_map", &self.flow_arena.node_map)
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
    /// ensures that you can get the required entity; 
    /// inserts if not in place. 
    pub fn entity_ensure(&mut self, id: &EntityId) -> &mut Entity {
        if !self.flow_arena.node_map.contains_key(id) {
            self.entity_insert(Entity::new_id(id));
        }
        self.entity_get_mut(id).expect("contains key")
    }
    pub fn entity_devote(&mut self, obj: EntityId, owner: EntityId, idx: usize) {
        self.flow_arena.devote(&obj, &owner, idx).ok();
    }
    pub fn entity_devote_push(&mut self, obj: EntityId, owner: EntityId) {
        self.flow_arena.devote_push(&obj, &owner).ok();
    }
    pub fn entity_grow_devote(&mut self, owner: EntityId, idx: usize) -> EntityId {
        let obj = self.entity_grow();
        self.entity_devote(obj, owner, idx);
        obj
    }
    pub fn entity_duplicate(&mut self, obj: EntityId, dude: EntityId) {
        let dude = self.entity_get(&dude).cloned().unwrap_or_default();
        self.entity_get_mut(&obj).map(|obj| {
            obj.duplicate_from(&dude)
        });
    }
    /// final product function: entity_add, duplicates its dude 
    /// and devotes to its owner.
    pub fn entity_add(&mut self, dude: EntityId , owner: EntityId, idx: usize) {
        let obj = self.entity_grow_devote(owner, idx);
        self.entity_duplicate(obj, dude)
    }
    /// removes entity from a flow_arena
    pub fn entity_decay(&mut self, id: &EntityId) {
        self.flow_arena.decay(id).ok();
        self.glass_refresh();
    }
}

/// flow_arena inquiry
impl Vessel {
    pub fn node(&self, id: &EntityId) -> Option<&EntityNode> {
        self.flow_arena.node(id)
    }
    pub fn entity_list(&self, id: &EntityId) -> Vec<&Entity> {
        let vec = self.flow_arena.node_map.get(id).map(|x| x.children.clone()).unwrap_or_default();
        
        vec.into_iter().filter_map(|id| self.flow_arena.node_map.get(&id)).map(|x| &x.entity).collect()
    }
    /// get all entity_ids
    pub fn entity_id_all(&self) -> Vec<EntityId> {
        self.flow_arena.entities().map(|x|x.id().clone()).collect()
    }
    /// Todo: get all entity_ids under id recrusively
    pub fn entity_ids(&self, _id: &EntityId) -> Vec<EntityId> {
        self.flow_arena.entities().map(|x|x.id().clone()).collect()
    }
    /// search all entities for "face" match
    pub fn entity_face_filter(&self, face: String) -> Vec<EntityId> {
        self.flow_arena.entities().filter_map(|x| {
            if x.face == face { Some(x.id().clone()) } else { None }
        }).collect()
    }
    pub fn entity_get(&self, id: &EntityId) -> Option<&Entity> {
        self.flow_arena.node_map.get(id).map(|x| &x.entity)
    }
    pub fn entity_get_mut(&mut self, id: &EntityId) -> Option<&mut Entity> {
        self.flow_arena.node_map.get_mut(id).map(|x| &mut x.entity)
    }
}

/// indents
impl Vessel {
    /// requires the id and its idx in node.children
    pub fn entity_dive(&mut self, id: EntityId, idx: usize) {
        if idx == 0 { return }
        let owner = {
            if let Some(owner) = if let Some(node) = self.node(&id) {
                node.parent
            } else { return } {
                owner
            } else { return }
        };
        let des = {
            if let Some(node) = self.node(&owner) {
                node.children[idx - 1]
            } else { return } 
        };
        self.entity_devote_push(id, des)
    }

}


impl Vessel {
    pub fn get_cube_vec(&self) -> Vec<Cube> {
        self.glass.get_cube_vec(self.router)
    }
    pub fn glass_refresh(&mut self) {
        self.glass.refresh(&self.flow_arena)
    }
}

impl Vessel {
    pub fn concise_debug_string(&self) -> String {
        let obj = self.flow_arena.root;
        concise_debug_impl(obj, self, 0)
    }
    pub fn concise_debug(&self) {
        println!("{}", self.concise_debug_string())
    }
}

fn concise_debug_impl(obj: EntityId, vessel: &Vessel, prefix: usize) -> String {
    let id_debug = vessel.entity_get(&obj).map_or(
        "".into(), 
        |x|{
            format!("{:?}: {:?}", x.id(), x.face)
        }
    );
    let children = vessel.node(&obj).map_or(
        Vec::new(), 
        |node| {
            node.children.clone()
        });
    let children_debug = children.iter().map(|x|
        concise_debug_impl(x.clone(), vessel, prefix+1)
    ).fold(String::new(), |x, y| {
        format!("{}\n{}", x, y)
    });
    let mut prefix_debug = " ".repeat(prefix*2);
    prefix_debug.push_str("|--");
    format!{"{}{}{}", prefix_debug, id_debug, children_debug}
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
    fn make_vessel(num: usize) -> (Vec<EntityId>, Vessel) {
        let mut vessel = Vessel::new();
        let id = (0..num).map(|_|
            vessel.entity_grow()
        ).collect();
        (id, vessel)
    }
    #[test]
    fn entity_list() {
        let (id, mut vessel) = make_vessel(5);
        vessel.flow_arena.devote_push(&id[1], &id[0]).ok();
        vessel.flow_arena.devote_push(&id[2], &id[0]).ok();
        vessel.flow_arena.devote_push(&id[3], &id[0]).ok();
        vessel.flow_arena.devote_push(&id[4], &id[0]).ok();
        vessel.flow_arena.devote_push(&id[4], &id[1]).ok();
        // 0_0 --> id --> [id1, id2, id3]
        //          `-------`-> id4
        vessel.entity_get_mut(&id[0]).map(|entity| entity.face = format!("Aloha!"));
        println!("{:#?}", vessel);
        println!("{:#?}", vessel.entity_get(&id[0]));
        println!("{:#?}", vessel.entity_list(&id[0]));
    }
    #[test]
    fn entity_decay() {
        let (id, mut vessel) = make_vessel(5);
        vessel.flow_arena.devote_push(&id[1], &id[0]).ok();
        vessel.flow_arena.devote_push(&id[2], &id[0]).ok();
        vessel.flow_arena.devote_push(&id[3], &id[0]).ok();
        vessel.flow_arena.devote_push(&id[4], &id[0]).ok();
        vessel.flow_arena.devote_push(&id[4], &id[1]).ok();
        // 0_0 --> id --> [id1, id2, id3]
        //          `-------`-> id4
        vessel.entity_get_mut(&id[0]).map(|entity| entity.face = format!("Aloha!"));
        vessel.entity_get_mut(&id[1]).map(|entity| entity.face = format!("Bobi."));
        vessel.entity_decay(&id[0]);
        println!("{:#?}", vessel);
    }
    #[test]
    fn serde() {
        let (id, mut vessel) = make_vessel(5);
        vessel.flow_arena.devote_push(&id[1], &id[0]).ok();
        vessel.flow_arena.devote_push(&id[2], &id[0]).ok();
        vessel.flow_arena.devote_push(&id[3], &id[0]).ok();
        vessel.flow_arena.devote_push(&id[4], &id[0]).ok();
        vessel.flow_arena.devote_push(&id[4], &id[1]).ok();
        // 0_0 --> id --> [id1, id2, id3]
        //          `-------`-> id4
        vessel.entity_get_mut(&id[0]).map(|entity| entity.face = format!("Aloha!"));
        vessel.entity_get_mut(&id[1]).map(|entity| entity.face = format!("Bobi."));
        let str = serde_json::to_string(&vessel).expect("failed to serialize vessel");
        println!("Serialize: {}", str);
        let vessel: Vessel = serde_json::from_str(&str).expect("failed to deserialize");
        println!("{:#?}", vessel);
    }

    fn retrive_random(id: &Vec<EntityId>) -> Option<EntityId> {
        let mut idx: usize = rand::random();
        if id.len() == 0 {
            None
        } else {
            idx %= id.len();
            id.get(idx).cloned()
        }
    }
    #[test] 
    fn random_demon_tests() {
        let length = 20000;
        let quiet = true;
        let func_set = [
            |(id, vessel): (&mut Vec<EntityId>, &mut Vessel)| {
                print!("Grow. ");
                let obj = vessel.entity_grow();
                println!("{:?}", obj);
                id.push(obj);
            },
            |(id, vessel): (&mut Vec<EntityId>, &mut Vessel)| {
                let obj = retrive_random(&id);
                let owner = retrive_random(&id);
                match (obj, owner) {
                    (Some(obj), Some(owner)) => {
                        println!("Devote. {:?} -> {:?}", obj, owner);
                        vessel.entity_devote_push(obj, owner);
                    }
                    _ => ()
                }
            },
            |(id, vessel): (&mut Vec<EntityId>, &mut Vessel)| {
                let obj = retrive_random(&id);
                match obj {
                    Some(obj) => {
                        println!("Decay. {:?} ", obj);
                        vessel.entity_decay(&obj);
                        id.retain(|x| x != &obj);
                    }
                    _ => ()
                }
            },
        ];
        let mut id = Vec::new();
        let mut vessel = Vessel::new();
        let mut seq: Vec<usize> = vec! [0;10];
        seq.extend((0..length).map(|_| {
            let mut i: usize = rand::random();
            i %= func_set.len();
            i
        }).collect::<Vec<usize>>());
        for (i, op) in seq.into_iter().enumerate() {
            print!("#{} ", i);
            func_set[op]((&mut id, &mut vessel));
            if !quiet { vessel.concise_debug(); }
        }
    }
}
