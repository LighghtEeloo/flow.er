use std::{collections::HashSet, fmt::Debug};
use serde::{Serialize, Deserialize};
use flow_arena::{Direction, FlowArena, FlowBase, FlowError, FlowDevote, FlowMap, FlowNode, FlowShift, Node};

use crate::Filter;

use super::{Entity, EntityId, EntityIdFactory, Glass, Settings};

pub type EntityNode = FlowNode<EntityId, Entity>;
pub type EntityFlow = FlowArena<EntityId, FlowNode<EntityId, Entity>>;

#[derive(Default, Clone, Serialize, Deserialize)]
pub struct Vessel {
    pub(crate) flow: EntityFlow,
    factory: EntityIdFactory,
    #[serde(default)]
    pub glass: Glass,
    #[serde(default)]
    pub settings: Settings
}

impl Vessel {
    pub fn new() -> Self {
        Self {
            flow: FlowArena::new(),
            factory: EntityIdFactory::default(),
            glass: Glass::default(),
            settings: Settings::default()
        }
    }
}

impl Debug for Vessel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Vessel")
         .field("glass", &self.glass)
         .field("id_factory", &self.factory)
         .field("flow_arena", &self.flow)
         .finish()
    }
}

/// flow_arena inquiry
impl Vessel {
    pub fn orphan(&self) -> Vec<EntityId> {
        self.flow.orphan()
    }
    pub fn node(&self, id: &EntityId) -> Option<&EntityNode> {
        self.flow.node(id)
    }
    /// get all entity_ids
    pub fn entity_id_all(&self) -> Vec<EntityId> {
        self.flow.entities().map(|x|x.id().clone()).collect()
    }
    /// get all entity_ids under id directly
    pub fn entity_id_direct(&self, obj: &EntityId) -> Vec<EntityId> {
        self.flow.node(obj).map_or(Vec::new(), |x| x.children())
    }
    /// get all entity_ids under id recrusively
    pub fn entity_offspring(&self, obj: &EntityId) -> HashSet<EntityId> {
        self.flow.node_offspring_set(obj)
    }
    pub fn entity_ownership(&self, obj: &EntityId) -> HashSet<EntityId> {
        self.flow.node_ownership_set(obj)
    }
    /// pick entities with all filters satisfied
    pub fn entity_pick_by(&self, filters: &Vec<Filter>) -> Vec<EntityId> {
        self.flow.entities().filter(|x| {
            x.pick_by(filters)
        }).map(|x| x.id().clone() ).collect()
    }
    /// take entities with any filter satisfied
    pub fn entity_match_by(&self, filters: &Vec<Filter>) -> Vec<EntityId> {
        self.flow.entities().filter(|x| {
            filters.iter().fold(true, |is, fil|
                is || x.pick_by(&vec![fil.clone()])
            )
        }).map(|x| x.id().clone() ).collect()
    }
    /// filter out all entities with filters
    pub fn entity_filter_out(&self, filters: &Vec<Filter>) -> Vec<EntityId> {
        self.flow.entities().filter(|x| {
            x.filter_out(filters)
        }).map(|x| x.id().clone() ).collect()
    }
    pub fn entity(&self, id: &EntityId) -> Option<&Entity> {
        self.flow.node(id).map(|x| &x.entity)
    }
    pub fn entity_mut(&mut self, id: &EntityId) -> Option<&mut Entity> {
        self.flow.node_mut(id).map(|x| &mut x.entity)
    }
}

/// flow_arena flow operation
impl Vessel {
    /// ```
    /// # use flow_vessel::*;
    /// let mut vessel = Vessel::new();
    /// let id = vessel.entity_grow().unwrap();
    /// println!("{:?}", vessel.entity_mut(&id));
    /// ```
    pub fn entity_grow(&mut self) -> Result<EntityId, FlowError> {
        let entity = Entity::new_rotate(&mut self.factory);
        self.entity_flow_grow(entity)
    }
    fn entity_flow_grow(&mut self, entity: Entity) -> Result<EntityId, FlowError> {
        let id = entity.id().clone();
        self.flow.grow(FlowNode::from_id(id.clone(), entity))?;
        Ok(id)
    }
    // /// ensures that you can get the required entity; 
    // /// inserts if not in place. 
    // pub fn entity_ensure(&mut self, id: &EntityId) -> &mut Entity {
    //     if self.flow.node(id).is_none() {
    //         self.entity_insert(Entity::new_id(id));
    //     }
    //     self.entity_mut(id).expect("contains key")
    // }
    fn entity_devote(&mut self, obj: EntityId, owner: EntityId, idx: usize) -> Result<(), FlowError> {
        self.flow.devote(&obj, &owner, idx)
    }
    pub fn entity_grow_devote(&mut self, owner: EntityId, idx: usize) -> Result<EntityId, FlowError> {
        let obj = self.entity_grow()?;
        self.entity_devote(obj, owner, idx)?;
        Ok(obj)
    }
    fn entity_duplicate(&mut self, obj: EntityId, dude: EntityId) {
        let dude = self.entity(&dude).cloned().unwrap_or_default();
        self.entity_mut(&obj).map(|obj| {
            obj.duplicate_from(&dude)
        });
    }
    /// final product function: entity_add, duplicates its dude 
    /// and devotes to its owner.
    pub fn entity_add(&mut self, dude: EntityId , owner: EntityId, idx: usize) -> Result<EntityId, FlowError> {
        let obj = self.entity_grow_devote(owner, idx)?;
        self.entity_duplicate(obj, dude);
        Ok(obj)
    }
    /// removes entity from a flow_arena
    pub fn entity_remove(&mut self, obj: &EntityId) -> Result<(), FlowError> {
        self.flow.decay(&obj)?;
        self.flow.erase(obj)?;
        self.glass_refresh();
        Ok(())
    }
}

/// indent & move
impl Vessel {
    pub fn entity_shuttle(&mut self, obj: &EntityId, dir: Direction) -> Result<EntityId, FlowError> {
        self.flow.shuttle(obj, dir)
    }

    /// using flow.migrate_iter
    pub fn entity_migrate(&mut self, obj: &EntityId, dir: Direction) -> Result<(), FlowError> {
        self.flow.migrate_iter(obj, dir)
    }
}

/// glass
impl Vessel {
    pub fn glass_refresh(&mut self) {
        self.glass.refresh(&self.flow, &self.settings.workspace_mode)
    }
}

/// flow debug prints
impl Vessel {
    pub fn concise_debug_string(&self) -> String {
        let objs = self.flow.orphan();
        objs.into_iter().fold("".to_owned(), |debug_info, obj| {
            format!("{}\n{}", debug_info, concise_debug_impl(obj, self, 1))
        })
    }
    pub fn concise_debug(&self) {
        println!("{}", self.concise_debug_string())
    }
}

fn concise_debug_impl(obj: EntityId, vessel: &Vessel, prefix: usize) -> String {
    let id_debug = vessel.entity(&obj).map_or(
        "".into(), 
        |x|{
            format!("{:?}: {:?}", x.id(), x.face)
        }
    );
    let children = vessel.node(&obj).map_or(
        Vec::new(), 
        |node| {
            node.children()
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
    use flow_arena::{FlowError, FlowLink};

    use super::*;
    // #[test]
    // fn vessel() {
    //     let vessel: Vessel = Vessel::new();
    //     println!("{:#?}", vessel);
    // }
    // #[test]
    // fn entity() {
    //     let mut vessel = Vessel::new();
    //     let id = vessel.entity_grow();
    //     println!("{:#?}", vessel);
    //     println!("{:?}", vessel.entity_mut(&id));
    // }
    // #[test]
    // fn entity_ensure() {
    //     let mut vessel = Vessel::new();
    //     let _id = vessel.entity_grow();
    //     let id1 = vessel.factory.incr_id();
    //     vessel.entity_ensure(&id1);
    //     println!("{:#?}", vessel);
    // }
    fn make_vessel(num: usize) -> (Vec<EntityId>, Vessel) {
        let mut vessel = Vessel::new();
        let id = (0..num).filter_map(|_|
            vessel.entity_grow().ok()
        ).collect();
        (id, vessel)
    }
    fn has_err(place: &str, fe: Option<FlowError>) {
        if let Some(e) = fe {
            println!("error at {}: {:?}", place, e)
        }
    }

    #[test]
    fn entity_remove() -> Result<(), FlowError> {
        let (id, mut vessel) = make_vessel(5);
        has_err("devote 1 -> 0", vessel.flow.devote_push(&id[1], &id[0]).err());
        has_err("devote 2 -> 0", vessel.flow.devote_push(&id[2], &id[0]).err());
        has_err("devote 3 -> 0", vessel.flow.devote_push(&id[3], &id[0]).err());
        has_err("devote 4 -> 0", vessel.flow.devote_push(&id[4], &id[0]).err());
        has_err("decay 4", vessel.flow.decay(&id[4]).err());
        vessel.concise_debug();
        println!("{:#?}", vessel);
        // has_err("devote 4 -> 1", vessel.flow_arena.devote_push(&id[4], &id[1]).err());
        has_err("link 4 -> 0", vessel.flow.link_push(&id[4], &id[0]).err());
        // 0_0 --> id --> [id1, id2, id3]
        //          `-------`-> id4
        vessel.entity_mut(&id[0]).map(|entity| entity.face = format!("Aloha!"));
        vessel.entity_mut(&id[1]).map(|entity| entity.face = format!("Bobi."));
        vessel.entity_remove(&id[0])?;
        println!("{:#?}", vessel);
        Ok(())
    }
    #[test]
    fn node_offspring() {
        let (id, mut vessel) = make_vessel(5);
        for i in 0..5 {
            println!("{}: {:?}", i, id[i]);
        }
        has_err("", vessel.flow.devote_push(&id[1], &id[0]).err());
        has_err("", vessel.flow.devote_push(&id[2], &id[0]).err());
        has_err("", vessel.flow.devote_push(&id[3], &id[0]).err());
        has_err("", vessel.flow.devote_push(&id[4], &id[0]).err());
        has_err("", vessel.flow.devote_push(&id[4], &id[1]).err());
        has_err("", vessel.flow.devote_push(&id[3], &id[4]).err());
        // 0_0 --> id --> [id1, id2, id3]
        //          `-------`-> id4 -/
        println!("{:?}", vessel.flow.node_offspring_set(&id[0]));
    }
    #[test]
    fn serde() {
        let (id, mut vessel) = make_vessel(5);
        vessel.flow.devote_push(&id[1], &id[0]).ok();
        vessel.flow.devote_push(&id[2], &id[0]).ok();
        vessel.flow.devote_push(&id[3], &id[0]).ok();
        vessel.flow.devote_push(&id[4], &id[0]).ok();
        vessel.flow.devote_push(&id[4], &id[1]).ok();
        // 0_0 --> id --> [id1, id2, id3]
        //          `-------`-> id4
        vessel.entity_mut(&id[0]).map(|entity| entity.face = format!("Aloha!"));
        vessel.entity_mut(&id[1]).map(|entity| entity.face = format!("Bobi."));
        let str = serde_json::to_string(&vessel).expect("failed to serialize vessel");
        println!("Serialize: {}", str);
        let vessel: Vessel = serde_json::from_str(&str).expect("failed to deserialize");
        println!("{:#?}", vessel);
    }

    fn retrive_random(id: &Vec<EntityId>) -> Option<usize> {
        let mut idx: usize = rand::random();
        if id.is_empty() {
            None
        } else {
            idx %= id.len();
            Some(idx)
        }
    }
    fn retrive_random_2(id: &Vec<EntityId>) -> Option<(usize, usize)> {
        loop {
            match (retrive_random(id), retrive_random(id)) {
                (Some(a), Some(b)) => {
                    if a != b { return Some((a, b)) }
                }
                (None, None) => return None,
                _ => panic!("What?")
            }
        }
    }
    #[test] 
    fn random_demon_tests() -> Result<(), FlowError> {
        let length = 4096;
        let quiet = true;
        let func_set = [
            // grow
            |(id, vessel): (&mut Vec<EntityId>, &mut Vessel)| -> Result<(), FlowError> {
                print!("Grow. ");
                let obj = vessel.entity_grow()?;
                println!("{:?}", obj);
                id.push(obj);
                Ok(())
            },
            // devote
            |(id, vessel): (&mut Vec<EntityId>, &mut Vessel)|  -> Result<(), FlowError>{
                let obj_owner = retrive_random_2(&id).map(|(i, j)| (id[i],id[j]));
                match obj_owner {
                    Some((obj, owner)) => {
                        println!("Devote. {:?} -> {:?}", obj, owner);
                        vessel.entity_devote(obj, owner, 0)?;
                    }
                    _ => ()
                }
                Ok(())
            },
            // remove
            |(id, vessel): (&mut Vec<EntityId>, &mut Vessel)| -> Result<(), FlowError> {
                let idx = retrive_random(&id);
                let obj = idx.map(|i| id[i]);
                match obj {
                    Some(obj) => {
                        println!("Erase. {:?} ", obj);
                        vessel.entity_remove(&obj)?;
                        id.remove(idx.unwrap());
                    }
                    _ => ()
                }
                Ok(())
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
            let res = func_set[op]((&mut id, &mut vessel)).err();
            res.map(|e| 
                println!("{:?}", e)
            );
            if !quiet { vessel.concise_debug(); }
        }
        vessel.concise_debug();
        Ok(())
    }
}
