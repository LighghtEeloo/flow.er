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

impl<Id: Clone + Hash + Eq + Default + Debug, Entity: Default + Debug> FlowArena<Id, Entity> {
    /// returns an iterator over all entities.
    pub fn entities(&mut self) -> Entities<Id, Entity> {
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
