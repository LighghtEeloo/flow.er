#[cfg(feature = "serde1")]
use serde::{Serialize, Deserialize};

use std::{collections::{HashMap, HashSet}, fmt::{self, Debug}, hash::Hash};

use super::*;

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
        let node_map = HashMap::new();
        FlowArena { node_map }
    }
    // pub fn spread<F>(&self, obj: &Id, fit: F) -> HashSet<Id> 
    // where F: Fn(Id) -> Option<Id> {
    //     let mut visit_set = HashSet::new();
    //     let mut final_set = HashSet::new();
    //     visit_set.insert(obj.clone());
    //     while !visit_set.is_empty() {
    //         let mut wait_set = HashSet::new();
    //         for obj in visit_set.iter() {
    //             let children = self.node_map.get(&obj)
    //                 .map(|x| x.children.clone() )
    //                 .unwrap_or_default();
    //             wait_set.extend(children);
    //         }
    //         final_set.extend(wait_set.iter().cloned());
    //         visit_set.clear();
    //         visit_set.extend(wait_set);
    //     }
    //     final_set
    // }
}


impl<Id, Entity> FlowBase for FlowArena<Id, Entity> 
where Id: Clone + Hash + Eq + Default + Debug, Entity: Default + Debug {
    type Id = Id;
    type Node = Node<Id, Entity>;

    fn orphan(&self) -> Vec<Self::Id> {
        self.node_map.iter().filter_map(|(id, node)|
            if node.parent.is_none() {
                Some(id.clone())
            } else {
                None
            }
        ).collect()
    }
    fn contains_node(&self, obj: &Self::Id) -> bool {
        self.node_map.contains_key(obj)
    }

    fn node(&self, obj: &Self::Id) -> Option<&Self::Node> {
        self.node_map.get(obj)
    }

    fn node_mut(&mut self, obj: &Self::Id) -> Option<&mut Self::Node> {
        self.node_map.get_mut(obj)
    }

    fn parent(&self, obj: &Self::Id) -> Option<Self::Id> {
        self.node(obj).map_or(None, |node| node.parent.clone())
    }

    fn children(&self, obj: &Self::Id) -> Vec<Self::Id> {
        self.node(obj).map_or(Vec::new(), |node| node.children.clone())
    }

    fn node_offspring_set(&self, obj: &Id) -> HashSet<Id> {
        let mut visit_set = HashSet::new();
        let mut final_set = HashSet::new();
        visit_set.insert(obj.clone());
        while !visit_set.is_empty() {
            let mut wait_set = HashSet::new();
            for obj in visit_set.iter() {
                let children = self.node(&obj)
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

    fn node_ownership_set(&self, obj: &Id) -> HashSet<Id> {
        let mut visit_set = HashSet::new();
        let mut final_set = HashSet::new();
        if self.contains_node(obj) {
            visit_set.insert(obj.clone());
            final_set.insert(obj.clone());
        }
        while !visit_set.is_empty() {
            let mut wait_set = HashSet::new();
            for obj in visit_set.iter() {
                let children = self.node(&obj)
                    .map(|x|  x.children.clone())
                    .unwrap_or_default();
                let set: Vec<Id> = children.into_iter().filter_map(|id| {
                    self.node(&id).map(|node| {
                        if node.parent == Some(obj.clone()) {
                            Some(id)
                        } else {
                            None
                        }
                    }).flatten()
                }).collect();
                wait_set.extend(set);
            }
            final_set.extend(wait_set.iter().cloned());
            visit_set.clear();
            visit_set.extend(wait_set);
        }
        final_set
    }
}

impl<Id, Entity> FlowLink for FlowArena<Id, Entity> 
where Id: Clone + Hash + Eq + Default + Debug, Entity: Default + Debug {
    fn link(&mut self, obj: &Self::Id, owner: &Self::Id, nth: usize) -> Result<(), FlowError> {
        if ! self.contains_node(obj) {
            return Err(FlowError::NotExistObj)
        } 
        let res = self.node_mut(owner).map(|owner| {
            if owner.children.contains(obj) {
                return Ok(())
            } 
            if nth > owner.children.len() {
                return Err(FlowError::InValidLen)
            } 
            owner.children.insert(nth, obj.clone());
            Ok(())
        }).unwrap_or(Err(FlowError::NotExistOwner));
        self.check_assert();
        res
    }

    fn link_push(&mut self, obj: &Self::Id, owner: &Self::Id) -> Result<(), FlowError> {
        let nth = self.node(owner).map_or(0, |node| node.children.len());
        let res = self.link(obj, owner, nth);
        self.check_assert();
        res
    }

    fn detach(&mut self, obj: &Self::Id, owner: &Self::Id) -> Result<(), FlowError> {
        if ! self.contains_node(obj) {
            return Err(FlowError::NotExistObj)
        } 
        let res = self.node_mut(owner).map(|owner| {
            if ! owner.children.contains(obj) {
                return Err(FlowError::AbandonedChild)
            }
            owner.children.retain(|x| x != obj);
            Ok(())
        }).unwrap_or(Err(FlowError::NotExistOwner));
        self.check_assert();
        res
    }
}


impl<Id, Entity> FlowMaid for FlowArena<Id, Entity> 
where Id: Clone + Hash + Eq + Default + Debug, Entity: Default + Debug {
    fn grow(&mut self, obj: Self::Node) -> Result<Self::Id, FlowError> {
        let res = if self.contains_node(obj.id()) {
            Err(FlowError::ExistGrow)
        } else {
            let id = obj.id().clone();
            self.node_map.insert(obj.id().clone(), obj);
            Ok(id)
        };
        self.check_assert();
        res
    }

    fn devote(&mut self, obj: &Self::Id, owner: &Self::Id, nth: usize) -> Result<(), FlowError> {
        let res = self.link(obj, owner, nth).and_then(|_| {
            self.node_mut(obj).map(|obj| {
                obj.parent = Some(owner.clone());
                Ok(())
            }).unwrap_or(Err(FlowError::NotExistObj))
        });
        self.check_assert();
        res
    }

    fn devote_push(&mut self, obj: &Self::Id, owner: &Self::Id) -> Result<(), FlowError> {
        let res = self.link_push(obj, owner).and_then(|_| {
            self.node_mut(obj).map_or(
                Err(FlowError::NotExistObj), 
                |obj| {
                    obj.parent = Some(owner.clone());
                    Ok(())
                }
            )
        });
        self.check_assert();
        res
    }

    fn decay(&mut self, obj: &Self::Id) -> Result<(), FlowError> {
        let owner = self.node(obj)
            .map(|x| {
                x.parent.clone() 
            });
        // Not owned by anyone
        if let Some(None) = owner {
            return Ok(())
        }
        let owner = owner.flatten();
        let res = self.node_mut(obj).map_or(
            Err(FlowError::NotExistObj),
            |obj| {
                obj.parent = None;
                Ok(())
            }
        ).and_then(|_| {
            owner.map_or(
                Err(FlowError::NotExistOwner), 
                |owner| {
                    self.detach(obj, &owner)
                }
            )
        });
        self.check_assert();
        res
    }

    fn erase(&mut self, obj: &Self::Id) -> Result<(), FlowError> {
        if ! self.contains_node(obj) {
            return Err(FlowError::NotExistObj)
        }
        let kill_set = self.node_ownership_set(obj);
        self.node_map.retain(|id, _| {
            ! kill_set.contains(id)
        });
        let () = self.node_map.values_mut().map(|obj| {
            obj.children.retain(|id| {
                ! kill_set.contains(id)
            })
        }).collect();
        self.check_assert();
        Ok(())
    }
}

impl<Id, Entity> FlowDock for FlowArena <Id, Entity>
where Id: Clone + Hash + Eq + Default + Debug, Entity: Default + Debug {
    fn dock(&mut self, owner: &Self::Id, flow: Self) -> Result<(), FlowError> {
        todo!()
    }

    fn undock(&mut self, obj: &Self::Id) -> Result<Self, FlowError> {
        todo!()
    }

    fn snap(&self, obj: &Self::Id) -> Result<Self, FlowError> {
        todo!()
    }
}

impl<Id, Entity> FlowShift for FlowArena <Id, Entity>
where Id: Clone + Hash + Eq + Default + Debug, Entity: Default + Debug {
    fn shuttle(&self, obj: &Self::Id, dir: Direction) -> Result<Self::Id, FlowError> {
        todo!()
    }

    fn migrate(&self, obj: &Self::Id, dir: Direction) -> Result<(), FlowError> {
        todo!()
    }

    fn migrate_iter(&self, obj: &Self::Id, dir: Direction) -> Result<(), FlowError> {
        todo!()
    }
}


impl<Id, Entity> Flow for FlowArena<Id, Entity> 
where Id: Clone + Hash + Eq + Default + Debug, Entity: Default + Debug {
    fn check(&self) -> Result<(), (FlowError, String)> {
        for (id, node) in self.node_map.iter() {
            let current_str = format!(", current: \nid: {:?}, \nnode: {:#?}", id, node);
            if id.clone() != node.id { return Err((FlowError::NodeIdNotMatch, current_str)) }
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
}


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

    fn wrapper<T>(name: &str, res: Result<T, FlowError>, flow: &FlowEntity, aloud: bool) {
        if aloud {
            let is_ok = res.is_ok();
            println!("{}: {}", name, if res.is_ok() {format!("success")} else {format!("{:?}", res.err())});
            assert!(is_ok);
            println!("{:#?}", flow);
        }
    }

    fn make_flow(aloud: bool) -> (FlowEntity, Vec<EntityId>) {
        let mut flow: FlowEntity = FlowArena::new();
        let obj_vec: Vec<NodeEntity> = (0..21).map(|x| 
            Node::from_id(x.clone().into(), ())
        ).collect();
        wrapper("Grow", flow.grow(obj_vec[0].clone()), &flow, aloud);
        wrapper("Grow", flow.grow(obj_vec[1].clone()), &flow, aloud);
        wrapper("Grow", flow.grow(obj_vec[2].clone()), &flow, aloud);
        wrapper("Grow", flow.grow(obj_vec[3].clone()), &flow, aloud);
        wrapper("Grow", flow.grow(obj_vec[4].clone()), &flow, aloud);
        wrapper("Grow", flow.grow(obj_vec[5].clone()), &flow, aloud);
        wrapper("Grow", flow.grow(obj_vec[6].clone()), &flow, aloud);
        wrapper("Grow", flow.grow(obj_vec[7].clone()), &flow, aloud);
        wrapper("Grow", flow.grow(obj_vec[8].clone()), &flow, aloud);
        wrapper("Grow", flow.grow(obj_vec[9].clone()), &flow, aloud);
        wrapper("Devote 2->0", flow.devote_push(obj_vec[2].id(), obj_vec[0].id()), &flow, aloud);
        wrapper("Devote 3->0", flow.devote_push(obj_vec[3].id(), obj_vec[0].id()), &flow, aloud);
        wrapper("Devote 4->0", flow.devote_push(obj_vec[4].id(), obj_vec[0].id()), &flow, aloud);
        wrapper("Devote 5->1", flow.devote_push(obj_vec[5].id(), obj_vec[1].id()), &flow, aloud);
        wrapper("Devote 6->1", flow.devote_push(obj_vec[6].id(), obj_vec[1].id()), &flow, aloud);
        wrapper("Devote 7->1", flow.devote_push(obj_vec[7].id(), obj_vec[1].id()), &flow, aloud);
        wrapper("Devote 8->1", flow.devote_push(obj_vec[8].id(), obj_vec[1].id()), &flow, aloud);
        wrapper("Devote 9->1", flow.devote_push(obj_vec[9].id(), obj_vec[1].id()), &flow, aloud);
        wrapper("Erase 3", flow.erase(obj_vec[3].id()), &flow, aloud);
        wrapper("Decay 1", flow.decay(obj_vec[1].id()), &flow, aloud);
        wrapper("Erase 1", flow.erase(obj_vec[1].id()), &flow, aloud);
        if cfg!(debug_assertions) && aloud { println!("Checked."); flow.check_assert() };
        (
            flow, 
            obj_vec.into_iter()
            .map(|node| node.id().clone())
            .collect()
        )
    }

    #[test]
    fn main_test() {
        make_flow(true);
    }

    #[test]
    fn ownership_offspring() {
        let (flow, obj_vec) = make_flow(true);
        println!("{:?}", flow.node_offspring_set(&obj_vec[0]));
        println!("{:?}", flow.node_ownership_set(&obj_vec[0]));
    }


    #[test]
    fn iter() {
        let (flow, _) = make_flow(false);
        wrapper("Print", Ok(()), &flow, true);
        let entities: Vec<()> = flow.entities().cloned().collect();
        println!("{:?}", entities);
        assert_eq!(entities, flow.node_map.values().map(|x| x.entity.clone()).collect::<Vec<()>>())
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
        let (flow, _) = make_flow(false);
        let str = serde_json::to_string(&flow).unwrap();
        print_wrapper(&str, true);
        let _flow: FlowEntity = serde_json::from_str(&str).unwrap();
        assert_eq!(flow, _flow)
    }
}
