use super::{Node, FlowArena};
use std::{fmt::Debug, hash::Hash};

#[derive(Clone)]
pub struct Entities<'a, Id: 'a, Entity: 'a> {
    iter: std::collections::hash_map::Values<'a, Id, Node<Id, Entity>>
}

impl<'a, Id, Entity> Iterator for Entities<'a, Id, Entity> {
    type Item = &'a Entity;

    fn next(&mut self) -> Option<Self::Item> {
        let x = self.iter.next();
        x.map(|node| &node.entity)
    }
}
pub struct EntitiesMut<'a, Id: 'a, Entity: 'a> {
    iter: std::collections::hash_map::ValuesMut<'a, Id, Node<Id, Entity>>
}

impl<'a, Id, Entity> Iterator for EntitiesMut<'a, Id, Entity> {
    type Item = &'a Entity;

    fn next(&mut self) -> Option<Self::Item> {
        let x = self.iter.next();
        x.map(|node| &node.entity)
    }
}

impl<Id, Entity> FlowArena<Id, Entity> 
where Id: Clone + Hash + Eq + Default + Debug, Entity: Default + Debug {
    /// returns an iterator over all entities.
    pub fn entities(&self) -> Entities<Id, Entity> {
        Entities {
            iter: self.node_map.values()
        }
    }
    /// returns an iterator over all entities.
    pub fn entities_mut(&mut self) -> EntitiesMut<Id, Entity> {
        EntitiesMut {
            iter: self.node_map.values_mut()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Flow;
    use serde::{Serialize, Deserialize};
    type FlowEntity = FlowArena<EntityId, String>;
    type NodeEntity = Node<EntityId, String>;
    #[derive(Clone, Default, Hash, PartialEq, Eq)]
    #[cfg_attr(feature = "serde1", derive(Serialize, Deserialize))]
    struct EntityId {
        idx: u64,
    }
    impl From<u64> for EntityId {
        fn from(idx: u64) -> Self {
            EntityId { idx }
        }
    }
    impl std::fmt::Debug for EntityId {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "[[{:?}]]", self.idx)
        }
    }

    fn wrapper(name: &str, res: bool, flow: &FlowEntity, aloud: bool) {
        if aloud {
            println!("{}: {}", name, if res {"success"} else {"error"});
            println!("{:#?}", flow);
        }
    }

    fn make_flow(aloud: bool) -> FlowEntity {
        let mut flow: FlowEntity = FlowArena::new();
        let obj_vec: Vec<NodeEntity> = (0..21).map(|x| Node::from_id(x.clone().into(), format!(":{}:", x.clone()))).collect();
        wrapper("Grow", flow.grow(obj_vec[1].clone()).is_ok(), &flow, aloud);
        wrapper("Grow", flow.grow(obj_vec[2].clone()).is_ok(), &flow, aloud);
        wrapper("Grow", flow.grow(obj_vec[3].clone()).is_ok(), &flow, aloud);
        wrapper("Grow", flow.grow(obj_vec[4].clone()).is_ok(), &flow, aloud);
        wrapper("Devote", flow.devote(obj_vec[4].id(), obj_vec[1].id(), 0).is_ok(), &flow, aloud);
        wrapper("Purge", flow.purge(obj_vec[1].id()).is_ok(), &flow, aloud);
        wrapper("Decay", flow.decay(obj_vec[1].id()).is_ok(), &flow, aloud);
        if cfg!(debug_assertions) && aloud { println!("Checked."); flow.check() };
        flow
    }

    #[test]
    fn iter() {
        let flow: FlowEntity = make_flow(false);
        wrapper("Print", true, &flow, true);
        let entities: Vec<String> = flow.entities().cloned().collect();
        println!("{:?}", entities);
        assert_eq!(entities, flow.node_map.values().map(|x| x.entity.clone()).collect::<Vec<String>>())
    }
}
