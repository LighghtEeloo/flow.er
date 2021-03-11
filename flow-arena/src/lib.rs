use std::{collections::HashMap, fmt::Debug, hash::Hash};
#[cfg(feature = "serde")]
use serde::{Serialize, Deserialize};

#[cfg(not(feature = "serde"))]
#[derive(Default)]
// Debug..
#[derive(Clone, PartialEq)]
pub struct Node<Id> {
    id: Id,
    parent: Option<Id>,
    children: Vec<Id>,
}
#[cfg(feature = "serde")]
#[derive(Default)]
#[derive(Serialize, Deserialize)]
// Debug..
#[derive(Clone, PartialEq)]
pub struct Node<Id> {
    id: Id,
    parent: Option<Id>,
    children: Vec<Id>,
}

impl<Id> Node<Id> {
    pub fn id(&self) -> &Id {
        &self.id
    }
    pub fn from_id(id: Id) -> Self {
        Node {
            id,
            parent: None,
            children: Vec::new(),
        }
    }
}

impl<Id: Debug> Debug for Node<Id> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct(format!("{:?}", self.id()).as_str())
            .field("parent", &self.parent)
            .field("children", &self.children)
            .finish()
    }
}

pub trait FlowLike {
    type Id;
    /// ensures root and returns it
    fn root(&mut self) -> &Node<Self::Id>;
    fn node(&self, obj: &Self::Id) -> Option<&Node<Self::Id>>;
    /// inserts obj to node_map; err if exist
    fn grow(&mut self, obj: Node<Self::Id>) -> Result<&Node<Self::Id>, ()>;
    /// link obj as a child of des at the nth place; err if nth > len or no obj / des
    fn devote(&mut self, obj: &Self::Id, des: &Self::Id, nth: usize) -> Result<(), ()>;
    fn devote_push(&mut self, obj: &Self::Id, des: &Self::Id) -> Result<(), ()>;
    /// removes from node_map and purges.
    fn decay(&mut self, obj: &Self::Id) -> Result<(), ()>;
    /// cuts all the links, but doesn't remove.
    fn purge(&mut self, obj: &Self::Id) -> Result<(), ()>;
}

#[cfg(not(feature = "serde"))]
// Debug..
#[derive(Debug, Clone)]
pub struct Flow<Id> {
    /// root: can be a Nil node or a dummy node, but must be in node_map.
    root: Id,
    node_map: HashMap<Id, Node<Id>>,
}

#[cfg(feature = "serde")]
#[derive(Serialize, Deserialize)]
// Debug..
#[derive(Debug, Clone)]
pub struct Flow<Id> {
    /// root: can be a Nil node or a dummy node, but must be in node_map.
    root: Id,
    node_map: HashMap<Id, Node<Id>>,
}

impl<Id: Clone + Hash + Eq + Default + Debug> Flow<Id> {
    pub fn new() -> Self {
        let node: Node<Id> = Node::default();
        let root = node.id().clone();
        let mut node_map = HashMap::new();
        node_map.insert(root.clone(), node);
        Flow { root, node_map }
    }
    /// panics if anything went wrong. Iff in debug state.
    #[cfg(debug_assertions)]
    fn check(&self) {
        for (id, node) in self.node_map.iter() {
            assert_eq!(id.clone(), node.id);
            if id.clone() == self.root {
                // root identical
                assert_eq!(Id::default(), self.root);
                // root has no parent
                assert_eq!(node.parent, None);
            } else {
                // nodes must have parent, except for root
                assert_ne!(node.parent, None);
            }
            // // children exist
            node.children.iter().for_each(|id| {
                assert!(self.node_map.get(id).is_some())
            });
            // parent exist
            if let Some(id) = node.parent.clone() {
                assert!(self.node_map.get(&id).is_some());
            }
        }
    } 
}

impl<Id: Clone + Hash + Eq + Default + Debug> FlowLike for Flow<Id> {
    type Id = Id;
    /// ensures root and returns it
    fn root(&mut self) -> &Node<Id> {
        if cfg!(debug_assertions) { self.check() };
        self.node_map.entry(Id::default()).or_default()
    }
    fn node(&self, obj: &Id) -> Option<&Node<Id>> {
        if cfg!(debug_assertions) { self.check() };
        self.node_map.get(obj)
    }
    fn grow(&mut self, mut obj: Node<Id>) -> Result<&Node<Id>, ()> {
        if cfg!(debug_assertions) { self.check() };
        obj.parent = Some(self.root.clone());
        if self.node_map.contains_key(obj.id()) {
            Err(())
        } else {
            let node = self.node_map.entry(obj.id().clone()).or_insert(obj);
            Ok(node)
        }
    }
    fn devote(&mut self, obj: &Id, des: &Id, nth: usize) -> Result<(), ()> {
        if cfg!(debug_assertions) { self.check() };
        if self.node_map.contains_key(obj) {
            self.node_map.get_mut(des)
                .map(|owner| {
                    if nth <= owner.children.len() {
                        owner.children.insert(nth, obj.clone());
                        Some(())
                    } else {
                        None
                    }
                })
                .flatten()
                .map(|x| {
                    self.node_map.get_mut(obj).map(|obj| {
                        obj.parent = Some(des.clone());
                    });
                    Some(x)
                })
                .flatten()
        } else { None } .ok_or(())
    }
    fn devote_push(&mut self, obj: &Id, des: &Id) -> Result<(), ()> {
        if cfg!(debug_assertions) { self.check() };
        let nth = self.node_map.get(des).map(|owner| owner.children.len());
        nth.map(|nth| {
            self.devote(obj, des, nth)
        }).unwrap_or(Err(()))
    }
    /// removes from node_map and purges.
    fn decay(&mut self, obj: &Id) -> Result<(), ()> {
        if cfg!(debug_assertions) { self.check() };
        self.node_map.remove(obj).map(|_|
            self.purge(obj).ok()
        ).flatten().ok_or(())
    }
    /// cuts all the links, but doesn't remove.
    fn purge(&mut self, obj: &Id) -> Result<(), ()> {
        if cfg!(debug_assertions) { self.check() };
        for (_, node) in self.node_map.iter_mut() {
            let root = self.root.clone();
            node.children.retain(|x| *x != *obj);
            node.parent = node
                .parent
                .clone()
                .and_then(|x| if x == *obj { Some(root) } else { Some(x) });
        }
        Ok(())
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::fmt;
    type FlowEntity = Flow<EntityId>;
    type NodeEntity = Node<EntityId>;
    #[derive(Clone, Default, Hash, PartialEq, Eq)]
    struct EntityId {
        idx: u64,
    }
    impl From<u64> for EntityId {
        fn from(idx: u64) -> Self {
            EntityId { idx }
        }
    }
    impl fmt::Debug for EntityId {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "[[{:?}]]", self.idx)
        }
    }

    #[test]
    fn main_test() {
        let wrapper = |name: &str, res: bool, flow: &FlowEntity| {
            println!("{}: {}", name, if res {"success"} else {"error"});
            println!("{:#?}", flow);
        };
        let mut flow: FlowEntity = Flow::new();
        let root = flow.root().id().clone();
        let obj_vec: Vec<NodeEntity> = (0..21).collect::<Vec<u64>>().iter_mut().map(|x| Node::from_id(x.clone().into())).collect();
        println!("{:?}", obj_vec.last());
        wrapper("Grow", flow.grow(obj_vec[1].clone()).is_ok(), &flow);
        wrapper("Grow", flow.grow(obj_vec[2].clone()).is_ok(), &flow);
        wrapper("Grow", flow.grow(obj_vec[3].clone()).is_ok(), &flow);
        wrapper("Grow", flow.grow(obj_vec[4].clone()).is_ok(), &flow);
        wrapper("Devote", flow.devote(obj_vec[1].id(), &root, 0).is_ok(), &flow);
        wrapper("Devote", flow.devote(obj_vec[2].id(), &root, 0).is_ok(), &flow);
        wrapper("DevotePush", flow.devote_push(obj_vec[3].id(), &root).is_ok(), &flow);
        wrapper("Devote", flow.devote(obj_vec[4].id(), obj_vec[1].id(), 0).is_ok(), &flow);
        wrapper("Purge", flow.purge(obj_vec[1].id()).is_ok(), &flow);
        wrapper("Decay", flow.decay(obj_vec[1].id()).is_ok(), &flow);
        if cfg!(debug_assertions) { println!("Checked."); flow.check() };
    }

    #[test]
    fn root() {
        let mut flow: FlowEntity = Flow::new();
        assert_eq!(flow.root().clone(), Node::default());
    }
}
