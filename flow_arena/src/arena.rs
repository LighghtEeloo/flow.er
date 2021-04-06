#[cfg(feature = "serde1")]
use serde::{Serialize, Deserialize};

use indexmap::IndexSet;
use std::{collections::{HashMap, HashSet}, fmt::{self, Debug}, hash::Hash};

use super::{FlowMap, FlowLink, FlowMaid, Flow, FlowError};

#[derive(Clone, Default)]
#[cfg_attr(feature = "serde1", derive(Serialize, Deserialize))]
#[cfg_attr(debug_assertions, derive(PartialEq))]
pub struct Node<Id, Entity> {
    id: Id,
    pub entity: Entity,
    /// indicates ownership.
    pub parent: Option<Id>,
    pub children: Vec<Id>,
}

pub type NodePure<Id> = Node<Id, ()>;

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

impl<Id: Debug + Clone, Entity: Debug> Debug for Node<Id, Entity> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct(format!("{:?}", self.id()).as_str())
            .field("<<", &self.parent.clone().map_or(
                format!("none"), 
                |x| format!("{:?}", x)
            ))
            .field(">>", &self.children)
            .field("::", &self.entity)
            .finish()
    }
}


#[derive(Clone)]
#[cfg_attr(debug_assertions, derive(PartialEq, Debug))]
pub struct FlowArena<Id: Hash + Eq + Clone, Entity> {
    pub(crate) node_map: HashMap<Id, Node<Id, Entity>>,
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
        let mut node_map = HashMap::new();
        FlowArena { node_map }
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
    pub(crate) fn check(&self) -> Result<(), (FlowError, String)> {
        for (id, node) in self.node_map.iter() {
            let current_str = format!(", current: \nid: {:?}, \nnode: {:#?}", id, node);
            if id.clone() != node.id { return Err((FlowError::NodeIdUnmatch, current_str)) }
            // children exist
            for id in node.children.iter() {
                if self.node_map.get(id).is_none() {
                    return Err((FlowError::NotExistChild, current_str));
                }
            }
            // parent exist
            if let Some(parent_id) = node.parent.clone() {
                let maybe = self.node_map.get(&parent_id);
                if maybe.is_none() {
                    return Err((FlowError::NotExistParent, current_str));
                }
                if let Some(node) = maybe {
                    if node.children.iter().find(|x| x.clone() == id).is_none() {
                        return Err((FlowError::AbandonedChild, current_str))
                    }
                }
            }
        }
        Ok(())
    }
    /// panics if anything went wrong. Iff in debug state.
    pub(crate) fn check_assert(&self) {
        if let Err((err, current)) = self.check() {
            panic!("{:?}{}", err, current)
        }   
    } 
}


impl<Id, Entity> FlowMap for FlowArena<Id, Entity> 
where Id: Clone + Hash + Eq + Default + Debug, Entity: Default + Debug {
    type Id = Id;
    type Node = Node<Id, Entity>;

    fn orphan(&self) -> Vec<Self::Id> {
        todo!()
    }

    fn node(&self, obj: &Self::Id) -> Option<&Self::Node> {
        todo!()
    }

    fn node_mut(&mut self, obj: &Self::Id) -> Option<&mut Self::Node> {
        todo!()
    }

    fn grow(&mut self, obj: Self::Node) -> Result<Self::Id, FlowError> {
        todo!()
    }
}

impl<Id, Entity> FlowLink for FlowArena<Id, Entity> 
where Id: Clone + Hash + Eq + Default + Debug, Entity: Default + Debug {
    fn link(&mut self, obj: &Self::Id, owner: &Self::Id, nth: usize) -> Result<(), FlowError> {
        todo!()
    }

    fn link_push(&mut self, obj: &Self::Id, owner: &Self::Id) -> Result<(), FlowError> {
        todo!()
    }

    fn detach(&mut self, obj: &Self::Id, owner: &Self::Id) -> Result<(), FlowError> {
        todo!()
    }
}


impl<Id, Entity> FlowMaid for FlowArena<Id, Entity> 
where Id: Clone + Hash + Eq + Default + Debug, Entity: Default + Debug {
    fn devote(&mut self, obj: &Self::Id, owner: &Self::Id, nth: usize) -> Result<(), FlowError> {
        todo!()
    }

    fn devote_push(&mut self, obj: &Self::Id, owner: &Self::Id) -> Result<(), FlowError> {
        todo!()
    }

    fn decay(&mut self, obj: &Self::Id) -> Result<(), FlowError> {
        todo!()
    }
    
    fn erase(&mut self, obj: &Self::Id) -> Result<Self::Node, FlowError> {
        todo!()
    }
}


impl<Id, Entity> Flow for FlowArena<Id, Entity> 
where Id: Clone + Hash + Eq + Default + Debug, Entity: Default + Debug {}

#[cfg(test)]
mod tests {
    use super::*;
    type FlowEntity = FlowPure<EntityId>;
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
            assert!(res);
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
        wrapper("Devote 4->1", flow.devote_push(obj_vec[4].id(), obj_vec[1].id()).is_ok(), &flow, aloud);
        wrapper("Devote 5->1", flow.devote_push(obj_vec[5].id(), obj_vec[1].id()).is_ok(), &flow, aloud);
        wrapper("Devote 6->1", flow.devote_push(obj_vec[6].id(), obj_vec[1].id()).is_ok(), &flow, aloud);
        wrapper("Devote 7->1", flow.devote_push(obj_vec[7].id(), obj_vec[1].id()).is_ok(), &flow, aloud);
        wrapper("Devote 8->1", flow.devote_push(obj_vec[8].id(), obj_vec[1].id()).is_ok(), &flow, aloud);
        wrapper("Devote 9->1", flow.devote_push(obj_vec[9].id(), obj_vec[1].id()).is_ok(), &flow, aloud);
        wrapper("Erase 3", flow.erase(obj_vec[3].id()).is_ok(), &flow, aloud);
        wrapper("Decay 1", flow.decay(obj_vec[1].id()).is_ok(), &flow, aloud);
        wrapper("Erase 1", flow.erase(obj_vec[1].id()).is_ok(), &flow, aloud);
        if cfg!(debug_assertions) && aloud { println!("Checked."); flow.check_assert() };
        flow
    }

    #[test]
    fn main_test() {
        make_flow(true);
    }

    #[test]
    fn offspring() {
        let flow = make_flow(false);
        // println!("{:?}", flow.node_offspring_list(&flow.root))
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
