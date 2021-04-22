#[cfg(feature = "serde1")]
use serde::{Serialize, Deserialize};

use std::{
    collections::{HashMap, HashSet}, 
    fmt::{self, Debug}, 
    hash::Hash
};

use super::*;

#[derive(Clone, Default)]
#[cfg_attr(feature = "serde1", derive(Serialize, Deserialize))]
#[cfg_attr(debug_assertions, derive(PartialEq))]
pub struct FlowNode<Id, Entity> {
    id: Id,
    pub entity: Entity,
    /// indicates ownership.
    parent: Option<Id>,
    children: Vec<Id>,
}

impl<Id, Entity> FlowNode<Id, Entity> {
    pub fn from_id(id: Id, entity: Entity) -> Self {
        FlowNode {
            id,
            entity,
            parent: None,
            children: Vec::new(),
        }
    }
}

impl<Id, Entity> Debug for FlowNode<Id, Entity> 
where Id: Debug + Clone, Entity: Debug {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct(format!("{:?}", self.id()).as_str())
            .field("<<", &self.parent.clone().map_or(
                format!(""), 
                |x| format!("{:?}", x)
            ))
            .field(">>", &self.children)
            .field("::", &self.entity)
            .finish()
    }
}

impl<Id, Entity> Node<Id> for FlowNode<Id, Entity> 
where Id: Clone {
    fn id(&self) -> &Id {
        &self.id
    }

    fn parent(&self) -> Option<Id> {
        self.parent.clone()
    }

    fn parent_set(&mut self, id: Id) {
        self.parent = Some(id)
    }

    fn parent_set_none(&mut self) {
        self.parent = None
    }

    fn children(&self) -> Vec<Id> {
        self.children.clone()
    }

    fn children_ref_mut(&mut self) -> &mut Vec<Id> {
        &mut self.children
    }
}


#[derive(Clone)]
#[cfg_attr(debug_assertions, derive(PartialEq, Debug))]
pub struct FlowArena<Id: Hash + Eq + Clone, FlowNode: Node<Id> + Clone> {
    pub(crate) node_map: HashMap<Id, FlowNode>,
}

impl<Id, FlowNode> Default for FlowArena<Id, FlowNode> 
where Id: Clone + Hash + Eq + Default + Debug, FlowNode: Node<Id> + Default + Debug + Clone {
    fn default() -> Self {
        Self::new()
    }
}

impl<Id, FlowNode> FlowArena<Id, FlowNode> 
where Id: Clone + Hash + Eq + Default + Debug, FlowNode: Node<Id> + Default + Debug + Clone {
    pub fn new() -> Self {
        let node_map = HashMap::new();
        FlowArena { node_map }
    }
}


impl<Id, FlowNode> FlowBase for FlowArena<Id, FlowNode> 
where Id: Clone + Hash + Eq + Default + Debug, FlowNode: Node<Id> + Default + Debug + Clone {
    type Id = Id;
    type Node = FlowNode;

    fn orphan(&self) -> Vec<Self::Id> {
        self.node_map.iter().filter_map(|(id, node)|
            if node.parent().is_none() {
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
}

impl<Id, FlowNode> FlowCheck for FlowArena<Id, FlowNode>
where Id: Clone + Hash + Eq + Default + Debug, FlowNode: Node<Id> + Default + Debug + Clone {
    fn check(&self) -> Result<(), (FlowError, String)> {
        for (id, node) in self.node_map.iter() {
            let current_str = format!(", current: \nid: {:?}, \nnode: {:#?}", id, node);
            if id != node.id() { return Err((FlowError::NodeIdNotMatch, current_str)) }
            // children exist
            for id in node.children().iter() {
                if self.node_map.get(id).is_none() {
                    return Err((FlowError::NotExistChild, current_str));
                }
            }
            // parent exist
            if let Some(parent_id) = node.parent().clone() {
                let maybe = self.node_map.get(&parent_id);
                if maybe.is_none() {
                    return Err((FlowError::NotExistParent, current_str));
                }
                if let Some(node) = maybe {
                    if node.children().iter().find(|x| x.clone() == id).is_none() {
                        return Err((FlowError::AbandonedChild, current_str))
                    }
                }
            }
        }
        Ok(())
    }
}

impl<Id, FlowNode> FlowMap for FlowArena<Id, FlowNode> 
where Id: Clone + Hash + Eq + Default + Debug, FlowNode: Node<Id> + Default + Debug + Clone {
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

    fn erase(&mut self, obj: &Self::Id) -> Result<(), FlowError> {
        if ! self.contains_node(obj) {
            return Err(FlowError::NotExistObj)
        }
        let kill_set = self.node_ownership_set(obj);
        self.node_map.retain(|id, _| {
            ! kill_set.contains(id)
        });
        let () = self.node_map.values_mut().map(|obj| {
            obj.children_ref_mut().retain(|id| {
                ! kill_set.contains(id)
            })
        }).collect();
        self.check_assert();
        Ok(())
    }
} 

impl<Id, FlowNode> FlowLink for FlowArena<Id, FlowNode> 
where Id: Clone + Hash + Eq + Default + Debug, FlowNode: Node<Id> + Default + Debug + Clone {}


impl<Id, FlowNode> FlowDevote for FlowArena<Id, FlowNode> 
where Id: Clone + Hash + Eq + Default + Debug, FlowNode: Node<Id> + Default + Debug + Clone {}


impl<Id, FlowNode> FlowDock for FlowArena <Id, FlowNode>
where Id: Clone + Hash + Eq + Default + Debug, FlowNode: Node<Id> + Default + Debug + Clone {
    fn dock(&mut self, owner: &Self::Id, mut vec: Vec<Self::Id>, mut flow: Self) -> Result<(), FlowError> {
        // check vec contains all flow.orphan()
        if vec.is_empty() {
            vec = flow.orphan()
        } else {
            let set: HashSet<Self::Id> = vec.iter().cloned().collect();
            if ! flow.orphan().iter().fold(true, |is_in, id| {
                is_in && set.contains(id)
            }) {
                return Err(FlowError::AbandonedChild)
            }
        }
        if flow.node_map.keys().fold(false, |is, id| {
            is || self.contains_node(id)
        }) { Err(FlowError::ExistDock) } else
        if ! self.contains_node(owner) {
            Err(FlowError::NotExistOwner)
        } else {
            self.node_mut(owner).map(|node| {
                node.children_ref_mut().extend(vec)
            });
            let _: Vec<()> = flow.node_map.iter_mut().map(|(_, node)| {
                node.parent_set(owner.clone())
            }).collect();
            self.node_map.extend(flow.node_map);
            self.check_assert();
            Ok(())
        }
    }

    fn undock_impl(&mut self, obj: &Self::Id, owned: bool) -> Result<(Self, Vec<Self::Id>), FlowError> {
        let (flow, vec) = if owned {
            self.snap_owned(obj)?
        } else { self.snap(obj)? };
        let set: HashSet<Self::Id> = flow.node_map.keys().cloned().collect();
        for id in set.iter() {
            let filter: Vec<()> = self.node_map.values()
                .filter(|node| {
                    ! set.contains(node.id()) && ! (node.id() == obj)
                })
                // node means current nodes excluding set_to_remove hereafter
                .filter_map(|node| {
                    // println!("set_id:{:?}, node_children:{:?}", id, node.children());
                    let ch_ft = node.children().iter()
                        .find(|&x| x == id).map(|_| ());
                    let pa_ft = node.parent().map(|x| { 
                        if &x == id { Some(()) } else { None }
                    }).flatten();
                    ch_ft.or(pa_ft)
                }).collect();
            if ! filter.is_empty() {
                return Err(FlowError::LinkedUndock)
            }
        }
        self.node_map.retain(|id, _| {
            ! set.contains(id)
        });
        self.node_mut(obj).map(|node| {
            node.children_ref_mut().clear()
        });
        self.check_assert();
        Ok((flow, vec))
    }

    fn snap(&self, obj: &Self::Id) -> Result<(Self, Vec<Self::Id>), FlowError> {
        if ! self.contains_node(obj) { return Err(FlowError::NotExistObj) }
        let vec = self.children(obj);
        let set = self.node_offspring_set(obj);
        let mut flow = FlowArena::new();
        let node_map: HashMap<Self::Id, Self::Node> = set.iter().cloned().filter_map(|id| {
            self.node_map.get(&id).cloned()
        }).map(|mut node| {
            if Some(obj.clone()) == node.parent() 
            || node.parent().clone().map_or(false, |id| ! set.contains(&id)) {
                node.parent_set_none()
            }
            (node.id().clone(), node)
        }).collect();
        flow.node_map.extend(node_map);
        self.check_assert();
        Ok((flow, vec))
    }

    fn snap_owned(&self, obj: &Self::Id) -> Result<(Self, Vec<Self::Id>), FlowError> {
        if ! self.contains_node(obj) { return Err(FlowError::NotExistObj) }
        let vec = self.children(obj);
        let mut set = self.node_ownership_set(obj);
        set.remove(obj); let set = set;
        let mut flow = FlowArena::new();
        let node_map: HashMap<Self::Id, Self::Node> = set.iter().cloned().filter_map(|id| {
            self.node_map.get(&id).cloned()
        }).map(|mut node| {
            if Some(obj.clone()) == node.parent() 
            || node.parent().clone().map_or(false, |id| ! set.contains(&id)) {
                node.parent_set_none()
            }
            (node.id().clone(), node)
        }).collect();
        flow.node_map.extend(node_map);
        self.check_assert();
        Ok((flow, vec))
    }
}

impl<Id, FlowNode> FlowShift for FlowArena <Id, FlowNode>
where Id: Clone + Hash + Eq + Default + Debug, FlowNode: Node<Id> + Default + Debug + Clone {}


impl<Id, FlowNode> Flow for FlowArena<Id, FlowNode> 
where Id: Clone + Hash + Eq + Default + Debug, FlowNode: Node<Id> + Default + Debug + Clone {}


#[derive(Clone)]
pub struct Entities<'a, Id: 'a, Entity: 'a> {
    iter: std::collections::hash_map::Values<'a, Id, FlowNode<Id, Entity>>
}

impl<'a, Id, Entity> Iterator for Entities<'a, Id, Entity> {
    type Item = &'a Entity;

    fn next(&mut self) -> Option<Self::Item> {
        let x = self.iter.next();
        x.map(|node| &node.entity)
    }
}
pub struct EntitiesMut<'a, Id: 'a, Entity: 'a> {
    iter: std::collections::hash_map::ValuesMut<'a, Id, FlowNode<Id, Entity>>
}

impl<'a, Id, Entity> Iterator for EntitiesMut<'a, Id, Entity> {
    type Item = &'a Entity;

    fn next(&mut self) -> Option<Self::Item> {
        let x = self.iter.next();
        x.map(|node| &node.entity)
    }
}

impl<Id, Entity> FlowArena<Id, FlowNode<Id, Entity>> 
where Id: Clone + Hash + Eq + Default + Debug, Entity: Default + Debug + Clone {
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
    type NodeEntity = FlowNode<EntityId, ()>;
    #[derive(Clone, Copy, Default, Hash, PartialEq, Eq)]
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

    /// before: 
    ///
    /// 0 - {2, 3 - {5, (6)}, 4}
    ///
    /// 1 - {7, 8, 9}
    ///
    /// after erase:
    /// 
    /// 0 - {2, 4}
    ///
    /// 6
    ///
    /// complex devote:
    ///
    /// 0 - {2, 4}
    ///
    /// 6 - {10, 11 - {12 - (14), (13)}, (12)}
    ///
    /// 13
    ///
    /// 14
    fn make_flow(aloud: bool) -> (FlowEntity, Vec<EntityId>) {
        let mut flow: FlowEntity = FlowArena::new();
        let obj_vec: Vec<NodeEntity> = (0..21).map(|x| 
            FlowNode::from_id(x.clone().into(), ())
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
        wrapper("Link 2->0", flow.link_push(obj_vec[2].id(), obj_vec[0].id()), &flow, aloud);
        wrapper("Devote 2->0", flow.devote_push(obj_vec[2].id(), obj_vec[0].id()), &flow, aloud);
        wrapper("Devote 3->0", flow.devote_push(obj_vec[3].id(), obj_vec[0].id()), &flow, aloud);
        wrapper("Devote 4->0", flow.devote_push(obj_vec[4].id(), obj_vec[0].id()), &flow, aloud);
        wrapper("Devote 5->3", flow.devote_push(obj_vec[5].id(), obj_vec[3].id()), &flow, aloud);
        wrapper("Link 6->3", flow.link_push(obj_vec[6].id(), obj_vec[3].id()), &flow, aloud);
        wrapper("Devote 7->1", flow.devote_push(obj_vec[7].id(), obj_vec[1].id()), &flow, aloud);
        wrapper("Devote 8->1", flow.devote_push(obj_vec[8].id(), obj_vec[1].id()), &flow, aloud);
        wrapper("Devote 9->1", flow.devote_push(obj_vec[9].id(), obj_vec[1].id()), &flow, aloud);
        // erase
        wrapper("Erase 3", flow.erase(obj_vec[3].id()), &flow, aloud);
        wrapper("Decay 1", flow.decay(obj_vec[1].id()), &flow, aloud);
        wrapper("Erase 1", flow.erase(obj_vec[1].id()), &flow, aloud);
        // complex
        wrapper("Grow", flow.grow(obj_vec[10].clone()), &flow, aloud);
        wrapper("Grow", flow.grow(obj_vec[11].clone()), &flow, aloud);
        wrapper("Grow", flow.grow(obj_vec[12].clone()), &flow, aloud);
        wrapper("Grow", flow.grow(obj_vec[13].clone()), &flow, aloud);
        wrapper("Grow", flow.grow(obj_vec[14].clone()), &flow, aloud);
        wrapper("Devote 10->6", flow.devote_push(obj_vec[10].id(), obj_vec[6].id()), &flow, aloud);
        wrapper("Devote 11->6", flow.devote_push(obj_vec[11].id(), obj_vec[6].id()), &flow, aloud);
        wrapper("Devote 12->11", flow.devote_push(obj_vec[12].id(), obj_vec[11].id()), &flow, aloud);
        wrapper("Link 12->6", flow.link_push(obj_vec[12].id(), obj_vec[6].id()), &flow, aloud);
        wrapper("Link 13->11", flow.link_push(obj_vec[13].id(), obj_vec[11].id()), &flow, aloud);
        wrapper("Link 14->12", flow.link_push(obj_vec[14].id(), obj_vec[12].id()), &flow, aloud);
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
    fn offspring_ownership() {
        let (flow, obj_vec) = make_flow(true);
        
        let offspring_impl = flow.node_offspring_set(&obj_vec[0]);
        let ownership_impl = flow.node_ownership_set(&obj_vec[0]);
        println!("offspring[0]: {:?}", offspring_impl);
        println!("ownership[0]: {:?}", ownership_impl);
        let offspring: HashSet<EntityId> = [
            obj_vec[2], 
            obj_vec[4]
        ].iter().cloned().collect();
        let ownership: HashSet<EntityId> = [
            obj_vec[0], 
            obj_vec[2], 
            obj_vec[4]
        ].iter().cloned().collect();
        assert_eq!(offspring, offspring_impl);
        assert_eq!(ownership, ownership_impl);

        let offspring_impl = flow.node_offspring_set(&obj_vec[6]);
        let ownership_impl = flow.node_ownership_set(&obj_vec[6]);
        println!("offspring[6]: {:?}", offspring_impl);
        println!("ownership[6]: {:?}", ownership_impl);
        let offspring: HashSet<EntityId> = [
            obj_vec[10], 
            obj_vec[11], 
            obj_vec[12], 
            obj_vec[13], 
            obj_vec[14], 
        ].iter().cloned().collect();
        let ownership: HashSet<EntityId> = [
            obj_vec[6], 
            obj_vec[10], 
            obj_vec[11], 
            obj_vec[12], 
        ].iter().cloned().collect();
        assert_eq!(offspring, offspring_impl);
        assert_eq!(ownership, ownership_impl);
    }

    #[test]
    fn flow_dock() {
        let (mut flow, obj_vec) = make_flow(false);
        let flow_ = flow.clone();

        let (sub_, vec_) = flow.snap(&obj_vec[6]).expect("snap error");
        println!("sub_flow[6]: {:#?}", sub_);
        println!("sub_vec[6]: {:?}", vec_);
        let mut flow_6 = flow_.clone();
        let (sub, vec) = flow_6.undock_impl(&obj_vec[6], false).expect("undock error");
        assert_eq!(sub, sub_);
        assert_eq!(vec, vec_);
        
        let (sub_, vec_) = flow.snap(&obj_vec[0]).expect("snap error");
        println!("sub_flow[0]: {:#?}", sub_);
        println!("sub_vec[0]: {:?}", vec_);
        let (sub, vec) = flow.undock_impl(&obj_vec[0], false).expect("undock error");
        assert_eq!(sub, sub_);
        assert_eq!(vec, vec_);
        flow.dock(&obj_vec[0], vec, sub).expect("dock error");
        // println!("current flow: {:#?}", flow);
        // println!("original flow: {:#?}", flow_);
        assert_eq!(flow, flow_);
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
        let node: NodeEntity = FlowNode::from_id(1.into(), ());
        print_wrapper(&serde_json::to_string(&node).unwrap(), false);
        let (flow, _) = make_flow(false);
        let str = serde_json::to_string(&flow).unwrap();
        print_wrapper(&str, true);
        let _flow: FlowEntity = serde_json::from_str(&str).unwrap();
        assert_eq!(flow, _flow)
    }
}
