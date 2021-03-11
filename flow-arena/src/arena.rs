use std::{collections::HashMap, fmt::Debug, hash::Hash};

#[cfg(feature = "serde1")]
use serde::{Serialize, Deserialize};

use crate::{Flow, FlowLike, Node};

#[cfg_attr(feature = "serde1", derive(Serialize, Deserialize))]
pub struct FlowArena<Id: Hash + Eq + Clone, Entity> {
    flow: Flow<Id>,
    map: HashMap<Id, Entity>
}

impl<Id: Hash + Eq + Clone, Entity> FlowArena<Id, Entity> {
    fn check(&self) {
        todo!()
    }
}

impl<Id: Clone + Hash + Eq + Default + Debug, Entity: Default> FlowLike for FlowArena<Id, Entity> {
    type Id = Id;
    type Node = (Node<Id>, Option<Entity>);
    type NodeRef = Node<Id>;

    // fn root<'a>(&'a mut self) -> &'a mut Self::Node {
    //     &mut (self.flow.root().to_owned(), None)
    // }
    // fn node<'a>(&'a self, obj: &Self::Id) -> Option<&'a Self::Node> {
    //     self.flow.node(obj).map(|node| {
    //         let node = node.to_owned();
    //         &(node, None)
    //     })
    // }
    // fn grow(&mut self, obj: Self::Node) -> Result<(), ()> {
    //     self.flow.grow(obj.0)
    // }

    fn root(&mut self) -> &mut Self::NodeRef {
        self.flow.root()
    }

    fn node(&self, obj: &Self::Id) -> Option<&Self::NodeRef> {
        self.flow.node(obj)
    }

    fn grow(&mut self, obj: Self::Node) -> Result<(), ()> {
        let id = obj.0.id().clone();
        self.flow.grow(obj.0)?;
        self.map.insert(id, obj.1.unwrap_or_default());
        Ok(())
    }

    fn devote(&mut self, obj: &Self::Id, des: &Self::Id, nth: usize) -> Result<(), ()> {
        self.flow.devote(obj, des, nth)
    }

    fn devote_push(&mut self, obj: &Self::Id, des: &Self::Id) -> Result<(), ()> {
        self.flow.devote_push(obj, des)
    }

    fn decay(&mut self, obj: &Self::Id) -> Result<(), ()> {
        self.flow.decay(obj)?;
        self.map.remove(obj);
        Ok(())
    }

    fn purge(&mut self, obj: &Self::Id) -> Result<(), ()> {
        self.flow.purge(obj)
    }

}
