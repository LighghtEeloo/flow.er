use std::{collections::HashMap, fmt::{self, Debug}, hash::Hash};

#[cfg(feature = "serde1")]
use serde::{Serialize, Deserialize};

#[derive(Clone, Default)]
#[cfg_attr(feature = "serde1", derive(Serialize, Deserialize))]
#[cfg_attr(debug_assertions, derive(PartialEq))]
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

#[cfg(debug_assertions)]
impl<Id: Debug> Debug for Node<Id> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct(format!("{:?}", self.id()).as_str())
            .field("parent", &self.parent)
            .field("children", &self.children)
            .finish()
    }
}

pub trait FlowLike {
    type Id;
    type Node;
    type NodeRef;
    /// ensures root and returns it; no check
    fn root(&mut self) -> &mut Self::NodeRef;
    // no check
    fn node(&self, obj: &Self::Id) -> Option<&Self::NodeRef>;
    /// inserts obj to node_map; err if exist
    fn grow(&mut self, obj: Self::Node) -> Result<(), ()>;
    /// link obj as a child of des at the nth place; err if nth > len or no obj / des
    fn devote(&mut self, obj: &Self::Id, des: &Self::Id, nth: usize) -> Result<(), ()>;
    fn devote_push(&mut self, obj: &Self::Id, des: &Self::Id) -> Result<(), ()>;
    /// removes from node_map and purges.
    fn decay(&mut self, obj: &Self::Id) -> Result<(), ()>;
    /// cuts all the links (except root), but doesn't remove.
    fn purge(&mut self, obj: &Self::Id) -> Result<(), ()>;
}


#[derive(Clone)]
#[cfg_attr(debug_assertions, derive(PartialEq, Debug))]
pub struct Flow<Id: Hash + Eq> {
    /// root: can be a Nil node or a dummy node, but must be in node_map.
    pub(crate) root: Id,
    pub(crate) node_map: HashMap<Id, Node<Id>>,
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
    pub(crate) fn check(&self) {
        for (id, node) in self.node_map.iter() {
            let current_str = format!(", current: \nid: {:?}, \nnode: {:#?}", id, node);
            assert_eq!(id.clone(), node.id);
            if id.clone() == self.root {
                // root identical
                assert_eq!(Id::default(), self.root, "! root identical {}", current_str);
                // root has no parent
                assert_eq!(node.parent, None, "! root non-parent {}", current_str);
            } else {
                // nodes must have parent, except for root
                assert_ne!(node.parent, None, "! nodes must have parent {}", current_str);
            }
            // children exist
            node.children.iter().for_each(|id| {
                assert!(self.node_map.get(id).is_some(), "! children exist {}", current_str)
            });
            // parent exist
            if let Some(parent_id) = node.parent.clone() {
                let maybe = self.node_map.get(&parent_id);
                assert!(maybe.is_some(), "! parent exist {}", current_str);
                if let Some(node) = maybe {
                    assert!(node.children.iter().find(|x| x.clone() == id).is_some(), "! parent has children {}", current_str)
                }
            }
        }
    } 
}

impl<Id: Clone + Hash + Eq + Default + Debug> FlowLike for Flow<Id> {
    type Id = Id;
    type Node = Node<Id>;
    type NodeRef = Node<Id>;
    /// ensures root and returns it
    fn root(&mut self) -> &mut Node<Id> {
        // no check because not necessarily clean
        self.node_map.entry(Id::default()).or_default()
    }
    fn node(&self, obj: &Id) -> Option<&Node<Id>> {
        // no check because no change
        self.node_map.get(obj)
    }
    fn grow(&mut self, mut obj: Node<Id>) -> Result<(), ()> {
        if cfg!(debug_assertions) { self.check() };
        obj.parent = Some(self.root.clone());
        match self.node_map.get(obj.id()) {
            Some(_) => Err(()),
            None => {
                let id = obj.id.clone();
                self.root().children.push(id.clone());
                self.node_map.insert(id, obj);
                Ok(())
            }
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
        self.purge(obj).ok().map(|_|
            self.node_map.remove(obj).map(|_|
                self.root().children.retain(|rooted| rooted != obj)
            )
        ).flatten().ok_or(())
    }
    /// cuts all the links (except root), but doesn't remove.
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
        // must be in root
        self.root().children.push(obj.clone());
        Ok(())
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    type FlowEntity = Flow<EntityId>;
    type NodeEntity = Node<EntityId>;
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
    impl fmt::Debug for EntityId {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
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
        let mut flow: FlowEntity = Flow::new();
        let obj_vec: Vec<NodeEntity> = (0..21).collect::<Vec<u64>>().iter_mut().map(|x| Node::from_id(x.clone().into())).collect();
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
    fn main_test() {
        make_flow(true);
    }

    #[test]
    fn root() {
        let mut flow: FlowEntity = Flow::new();
        assert_eq!(flow.root().clone(), Node::default());
    }

    #[test]
    fn serde() {
        let print_wrapper = |str: &String, aloud: bool| {
            if aloud {
                println!("{}",str)
            }
        };
        let id: EntityId = 1.into();
        print_wrapper(&serde_json::to_string(&id).unwrap(), false);
        let node: NodeEntity = Node::from_id(1.into());
        print_wrapper(&serde_json::to_string(&node).unwrap(), false);
        let flow = make_flow(false);
        let str = serde_json::to_string(&flow).unwrap();
        print_wrapper(&str, true);
        let _flow: FlowEntity = serde_json::from_str(&str).unwrap();
        assert_eq!(flow, _flow)
    }
}
