#[cfg(feature = "serde1")]
use serde::{Serialize, Deserialize};

use indexmap::IndexSet;
use std::{collections::{HashMap, HashSet}, fmt::{self, Debug}, hash::Hash};
use super::Flow;

#[derive(Clone, Default)]
#[cfg_attr(feature = "serde1", derive(Serialize, Deserialize))]
#[cfg_attr(debug_assertions, derive(PartialEq))]
pub struct Node<Id, Entity> {
    id: Id,
    pub entity: Entity,
    pub parent: Option<Id>,
    pub children: Vec<Id>,
}

impl<Id, Entity> Node<Id, Entity> {
    pub fn id(&self) -> &Id {
        &self.id
    }
    pub fn from_id(id: Id, entity: Entity) -> Self {
        Node {
            id,
            entity,
            parent: None,
            children: Vec::new(),
        }
    }
}

impl<Id: Debug, Entity: Debug> Debug for Node<Id, Entity> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct(format!("{:?}", self.id()).as_str())
            .field("parent", &self.parent)
            .field("children", &self.children)
            .field(":", &self.entity)
            .finish()
    }
}


#[derive(Clone)]
#[cfg_attr(debug_assertions, derive(PartialEq, Debug))]
pub struct FlowArena<Id: Hash + Eq, Entity> {
    /// root: can be a Nil node or a dummy node, but must be in node_map;    
    /// it could contain title info.
    /// 
    /// for now, only Id::default can be root.
    pub root: Id,
    pub node_map: HashMap<Id, Node<Id, Entity>>,
}

pub type FlowPure<Id> = FlowArena<Id, ()>;

impl<Id, Entity> Default for FlowArena<Id, Entity> 
where Id: Clone + Hash + Eq + Default + Debug, Entity: Default + Debug {
    fn default() -> Self {
        Self::new()
    }
}

impl<Id, Entity> FlowArena<Id, Entity> 
where Id: Clone + Hash + Eq + Default + Debug, Entity: Default + Debug {
    pub fn new() -> Self {
        let node: Node<Id, Entity> = Node::default();
        let root = node.id().clone();
        let mut node_map = HashMap::new();
        node_map.insert(root.clone(), node);
        FlowArena { root, node_map }
    }
    pub fn node_offspring_list(&self, obj: &Id) -> HashSet<Id> {
        let mut visit_set = HashSet::new();
        let mut final_set = HashSet::new();
        visit_set.insert(obj.clone());
        while !visit_set.is_empty() {
            let mut wait_set = HashSet::new();
            for obj in visit_set.iter() {
                let children = self.node_map.get(&obj)
                    .map(|x| x.children.clone() )
                    .unwrap_or_default();
                wait_set.extend(children);
            }
            final_set.extend(wait_set.iter().cloned());
            visit_set.clear();
            visit_set.extend(wait_set);
        }
        final_set
    }
    /// panics if anything went wrong. Iff in debug state.
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

impl<Id, Entity> Flow for FlowArena<Id, Entity> 
where Id: Clone + Hash + Eq + Default + Debug, Entity: Default + Debug {
    type Id = Id;
    type Node = Node<Id, Entity>;

    /// ensures root and returns it
    fn root(&mut self) -> &mut Node<Id, Entity> {
        // no check because not necessarily clean
        self.node_map.entry(Id::default()).or_default()
    }
    fn node(&self, obj: &Id) -> Option<&Node<Id, Entity>> {
        // no check because no change
        self.node_map.get(obj)
    }
    fn grow(&mut self, mut obj: Node<Id, Entity>) -> Result<(), ()> {
        let res = match self.node_map.get(obj.id()) {
            Some(_) => Err(()),
            None => {
                let id = obj.id.clone();
                self.root().children.push(id.clone());
                obj.parent = Some(self.root.clone());
                self.node_map.insert(id, obj);
                Ok(())
            }
        };
        if cfg!(debug_assertions) { self.check() };
        res
    }
    fn devote(&mut self, obj: &Id, des: &Id, nth: usize) -> Result<(), ()> {
        if self.node_offspring_list(obj).get(des).is_some() { return Err(()) }
        // Note: no obj in root.
        self.root().children.retain(|x| x != obj);
        let res = if self.node_map.contains_key(obj) {
            self.node_map.get_mut(des)
                .map(|owner| {
                    if nth <= owner.children.len() {
                        owner.children.insert(nth, obj.clone());
                        Some(())
                    } else { None }
                }).flatten()
                .map(|x| {
                    self.node_map.get_mut(obj).map(|obj| {
                        obj.parent = Some(des.clone());
                    });
                    Some(x)
                }).flatten()
        } else { None } .ok_or(());
        if cfg!(debug_assertions) { self.check() };
        res
    }
    fn devote_push(&mut self, obj: &Id, des: &Id) -> Result<(), ()> {
        let nth = self.node_map.get(des).map(|owner| owner.children.len());
        let res = nth.map(|nth| {
            self.devote(obj, des, nth)
        }).unwrap_or(Err(()));
        if cfg!(debug_assertions) { self.check() };
        res
    }
    // fn merge_flow(&mut self, flow: Self, des: &Self::Id, nth: usize) -> Result<(), ()> {
    //     if cfg!(debug_assertions) { self.check() };
    //     let collision = self.node_map.keys().any(|id| flow.node_map.contains_key(id));
    //     if collision {
    //         Err(())
    //     } else {
    //         let node_map = flow.node_map.into_iter();
    //         self.node_map.extend(node_map);
    //         // Todo: devote
    //         Ok(())
    //     }
    // }
    // fn merge_flow_push(&mut self, flow: Self, des: &Self::Id) -> Result<(), ()> {
    //     if cfg!(debug_assertions) { self.check() };
    //     let nth = self.node_map.get(des).map(|owner| owner.children.len());
    //     nth.map(|nth| {
    //         self.merge_flow(flow, des, nth)
    //     }).unwrap_or(Err(()))
    // }
    /// cuts all the links related to the obj and resets obj to root, 
    /// but doesn't remove.
    fn decay(&mut self, obj: &Id) -> Result<(), ()> {
        // Note: move children to parent.
        let mut orphan: Vec<Id> = Vec::new();
        let re_owner = self.node_map.get(obj)
            .map_or(None, |x| x.parent.clone())
            .unwrap_or(self.root.clone());
        for (_, node) in self.node_map.iter_mut() {
            let re_owner = re_owner.clone();
            node.children.retain(|x| x != obj);
            node.parent = node.parent.clone()
                .map(|parent| if &parent == obj { 
                    orphan.push(node.id.clone());
                    re_owner
                } else { parent });
        }
        // must be in root
        let root = self.root.clone();
        self.node_map.get_mut(obj).map(|node| {
            node.parent = Some(root)
        });
        self.root().children.retain(|x| x != obj);
        self.root().children.push(obj.clone());
        self.node_map.get_mut(&re_owner).map(|x| {
            let mut h: IndexSet<Id> = x.children.iter().cloned().collect();
            h.extend(orphan.into_iter());
            x.children = h.into_iter().collect();
        });
        if cfg!(debug_assertions) { self.check() };
        Ok(())
    }
    /// removes from node_map and purges.
    fn erase(&mut self, obj: &Id) -> Result<(), ()> {
        let res = if &self.root == obj {
            self.root().children.clear();
            self.node_map.retain(|k, _| k == obj);
            Ok(())
        } else {
            self.decay(obj).ok().map(|_|
                self.node_map.remove(obj).map(|_|
                    self.root().children.retain(|rooted| rooted != obj)
                )
            ).flatten().ok_or(())
        };
        if cfg!(debug_assertions) { self.check() };
        res
    }

    fn link(&mut self, obj: &Self::Id, des: &Self::Id, nth: usize) -> Result<(), ()> {
        todo!()
    }

    fn link_push(&mut self, obj: &Self::Id, des: &Self::Id) -> Result<(), ()> {
        todo!()
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    type FlowEntity = FlowArena<EntityId, ()>;
    type NodeEntity = Node<EntityId, ()>;
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
        let mut flow: FlowEntity = FlowArena::new();
        let obj_vec: Vec<NodeEntity> = (0..21).map(|x| Node::from_id(x.clone().into(), ())).collect();
        wrapper("Grow", flow.grow(obj_vec[1].clone()).is_ok(), &flow, aloud);
        wrapper("Grow", flow.grow(obj_vec[2].clone()).is_ok(), &flow, aloud);
        wrapper("Grow", flow.grow(obj_vec[3].clone()).is_ok(), &flow, aloud);
        wrapper("Grow", flow.grow(obj_vec[4].clone()).is_ok(), &flow, aloud);
        wrapper("Grow", flow.grow(obj_vec[5].clone()).is_ok(), &flow, aloud);
        wrapper("Grow", flow.grow(obj_vec[6].clone()).is_ok(), &flow, aloud);
        wrapper("Grow", flow.grow(obj_vec[7].clone()).is_ok(), &flow, aloud);
        wrapper("Grow", flow.grow(obj_vec[8].clone()).is_ok(), &flow, aloud);
        wrapper("Grow", flow.grow(obj_vec[9].clone()).is_ok(), &flow, aloud);
        wrapper("Devote 4->1", flow.devote(obj_vec[4].id(), obj_vec[1].id(), 0).is_ok(), &flow, aloud);
        wrapper("Devote 5->1", flow.devote(obj_vec[5].id(), obj_vec[1].id(), 0).is_ok(), &flow, aloud);
        wrapper("Devote 6->1", flow.devote(obj_vec[6].id(), obj_vec[1].id(), 0).is_ok(), &flow, aloud);
        wrapper("Devote 7->1", flow.devote(obj_vec[7].id(), obj_vec[1].id(), 0).is_ok(), &flow, aloud);
        wrapper("Devote 8->1", flow.devote(obj_vec[8].id(), obj_vec[1].id(), 0).is_ok(), &flow, aloud);
        wrapper("Devote 9->1", flow.devote(obj_vec[9].id(), obj_vec[1].id(), 0).is_ok(), &flow, aloud);
        wrapper("Erase 4", flow.erase(obj_vec[4].id()).is_ok(), &flow, aloud);
        wrapper("Purge 1", flow.decay(obj_vec[1].id()).is_ok(), &flow, aloud);
        wrapper("Erase 1", flow.erase(obj_vec[1].id()).is_ok(), &flow, aloud);
        if cfg!(debug_assertions) && aloud { println!("Checked."); flow.check() };
        flow
    }

    #[test]
    fn main_test() {
        make_flow(true);
    }

    #[test]
    fn root() {
        let mut flow: FlowEntity = FlowArena::new();
        assert_eq!(flow.root().clone(), Node::default());
    }
    #[test]
    fn offspring() {
        let flow = make_flow(false);
        println!("{:?}", flow.node_offspring_list(&flow.root))
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
        let node: NodeEntity = Node::from_id(1.into(), ());
        print_wrapper(&serde_json::to_string(&node).unwrap(), false);
        let flow = make_flow(false);
        let str = serde_json::to_string(&flow).unwrap();
        print_wrapper(&str, true);
        let _flow: FlowEntity = serde_json::from_str(&str).unwrap();
        assert_eq!(flow, _flow)
    }
}
