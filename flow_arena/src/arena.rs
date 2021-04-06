#[cfg(feature = "serde1")]
use serde::{Serialize, Deserialize};

use indexmap::IndexSet;
use std::{collections::{HashMap, HashSet}, fmt::{self, Debug}, hash::Hash};

use super::{FlowMap, FlowTree, FlowGraph, Flow, FlowError};

#[derive(Clone, Default)]
#[cfg_attr(feature = "serde1", derive(Serialize, Deserialize))]
#[cfg_attr(debug_assertions, derive(PartialEq))]
pub struct Node<Id, Entity> {
    id: Id,
    pub entity: Entity,
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
    /// root: can be a Nil node or a dummy node, but must be in node_map;    
    /// it could contain title info.
    /// 
    /// for now, only Id::default can be root.
    pub root: Id,
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
        let node: Node<Id, Entity> = Node::default();
        let root = node.id().clone();
        let mut node_map = HashMap::new();
        node_map.insert(root.clone(), node);
        FlowArena { root, node_map }
    }
    pub fn node_map(&self) -> &HashMap<Id, Node<Id, Entity>> {
        &self.node_map
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
        // has root
        if ! self.node_map.contains_key(&self.root) { return Err((FlowError::NoRoot, "".into())) }
        // root identical with default
        if self.root != Id::default() { return Err((FlowError::NotDefaultRoot, "".into())) }
        // root has no parent
        if let Some(Some(_)) | None = self.node_map.get(&self.root).map(|r| r.parent.clone()) {
            return Err((FlowError::ParentedRoot, "".into()))
        }
        for (id, node) in self.node_map.iter() {
            let current_str = format!(", current: \nid: {:?}, \nnode: {:#?}", id, node);
            if id.clone() != node.id { return Err((FlowError::NodeIdUnmatch, current_str)) }
            if id.clone() != self.root {
                // nodes must have parent, except for root
                if node.parent == None { return Err((FlowError::NoParent, current_str)) }
            }
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

    fn root(&mut self) -> &mut Node<Id, Entity> {
        // no check because not necessarily clean
        self.node_map.entry(Id::default()).or_default()
    }
    fn node(&self, obj: &Id) -> Option<&Node<Id, Entity>> {
        // no check because no change
        self.node_map.get(obj)
    }
    fn node_mut(&mut self, obj: &Id) -> Option<&mut Node<Id, Entity>> {
        // no check because no change
        self.node_map.get_mut(obj)
    }

    fn grow(&mut self, mut obj: Self::Node) -> Result<Self::Id, FlowError> {
        if self.node_map.contains_key(obj.id()) {
            Err(FlowError::ExistGrow)
        } else {
            let id = obj.id().clone();
            obj.parent = Some(self.root.clone());
            self.node_map.insert(id.clone(), obj);
            self.root().children.push(id.clone());
            if cfg!(debug_assertions) { self.check_assert() };
            Ok(id)
        }
    }

    fn erase(&mut self, obj: &Self::Id) -> Result<Self::Node, FlowError> {
        if ! self.root().children.contains(obj) {
            Err(FlowError::NotOrphaned)
        } else {
            self.root().children.retain(|x| x != obj);
            // let orphans = self.node(obj).map_or(Vec::new(), |x| x.children.clone());
            let res = self.node_map.remove(obj).ok_or(FlowError::NotExistObj); 
            if cfg!(debug_assertions) { self.check_assert() };
            res
        }
    }
}

impl<Id, Entity> FlowTree for FlowArena<Id, Entity> 
where Id: Clone + Hash + Eq + Default + Debug, Entity: Default + Debug {
    fn devote(&mut self, obj: &Self::Id, owner: &Self::Id, nth: usize) -> Result<(), FlowError> {
        if ! self.node_map.contains_key(obj) {
            Err(FlowError::NotExistObj)
        } else 
        if ! self.node_map.contains_key(owner) {
            Err(FlowError::NotExistOwner)
        } else 
        if ! self.root().children.contains(obj) {
            Err(FlowError::NotOrphaned)
        } else 
        if &self.root == owner || &self.root == obj {
            Err(FlowError::RootDevote)
        } else 
        {
            self.root().children.retain(|x| x != obj);
            {
                let owner_node = self.node_mut(owner).expect("checked");
                owner_node.children.retain(|x| x != obj);
                if nth > owner_node.children.len() { return Err(FlowError::InValidLen) }
                owner_node.children.insert(nth, obj.clone());
            }
            self.node_mut(obj).expect("checked").parent = Some(owner.clone());
            if cfg!(debug_assertions) { self.check_assert() };
            Ok(())
        }
    }

    fn devote_push(&mut self, obj: &Self::Id, owner: &Self::Id) -> Result<(), FlowError> {
        let nth = self.node(owner).map(|owner_node| 
            owner_node.children.len()
        );
        let res = nth.map(|nth| {
            self.devote(obj, owner, nth)
        }).unwrap_or(Err(FlowError::NotExistObj));
        if cfg!(debug_assertions) { self.check_assert() };
        res
    }

    fn decay(&mut self, obj: &Self::Id) -> Result<(), FlowError> {
        if ! self.node_map.contains_key(obj) {
            Err(FlowError::NotExistObj)
        } else 
        if ! self.root().children.contains(obj) {
            // Ok(())
            Err(FlowError::NotOrphaned)
        } else 
        if &self.root == obj {
            Err(FlowError::RootDecay)
        } else 
        {
            let mut orphan: Vec<Id> = Vec::new();
            let re_owner = self.node(obj)
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
            self.node_mut(obj).map(|node| {
                node.parent = Some(root);
                node.children.clear();
            });
            self.root().children.retain(|x| x != obj);
            self.root().children.push(obj.clone());
            self.node_mut(&re_owner).map(|x| {
                let mut h: IndexSet<Id> = x.children.iter().cloned().collect();
                h.extend(orphan.into_iter());
                x.children = h.into_iter().collect();
            });
            if cfg!(debug_assertions) { self.check_assert() };
            Ok(())
        }
    }
}


impl<Id, Entity> FlowGraph for FlowArena<Id, Entity> 
where Id: Clone + Hash + Eq + Default + Debug, Entity: Default + Debug {
    fn link(&mut self, obj: &Self::Id, owner: &Self::Id, nth: usize) -> Result<(), FlowError> {
        if ! self.node_map.contains_key(obj) {
            Err(FlowError::NotExistObj)
        } else 
        if ! self.node_map.contains_key(owner) {
            Err(FlowError::NotExistOwner)
        } else 
        if self.root().children.contains(obj) {
            Err(FlowError::IsOrphaned)
        } else 
        if &self.root == owner || &self.root == obj {
            Err(FlowError::RootLink)
        } else 
        {
            let owner = self.node_mut(owner)
                .expect("checked");
            if nth > owner.children.len() {
                Err(FlowError::InValidLen)
            } else {
                owner.children.insert(nth, obj.clone());
                Ok(())
            }
        }
    }

    fn link_push(&mut self, obj: &Self::Id, owner: &Self::Id) -> Result<(), FlowError> {
        let nth = self.node(owner).map(|owner_node| 
            owner_node.children.len()
        );
        let res = nth.map(|nth| {
            self.link(obj, owner, nth)
        }).unwrap_or(Err(FlowError::NotExistObj));
        if cfg!(debug_assertions) { self.check_assert() };
        res
    }

    fn detach(&mut self, obj: &Self::Id, owner: &Self::Id ) -> Result<(), FlowError> {
        if ! self.node_map.contains_key(obj) {
            Err(FlowError::NotExistObj)
        } else 
        if ! self.node_map.contains_key(owner) {
            Err(FlowError::NotExistOwner)
        } else 
        if self.root().children.contains(obj) {
            Err(FlowError::IsOrphaned)
        } else 
        if &self.root == owner || &self.root == obj {
            Err(FlowError::RootDetach)
        } else 
        {
            if Some(owner.clone()) == self.node(obj)
                .map_or(None, |x| x.parent.clone()) 
            {
                Err(FlowError::OwnerDetach)
            } else {
                let owner = self.node_mut(owner).expect("checked");
                owner.children.retain(|x| x != obj);
                Ok(())
            }
        }
    }

    fn defect(&mut self, obj: &Self::Id, owner: &Self::Id) -> Result<(), FlowError> {
        if ! self.node_map.contains_key(obj) {
            Err(FlowError::NotExistObj)
        } else 
        if ! self.node_map.contains_key(owner) {
            Err(FlowError::NotExistOwner)
        } else 
        if self.root().children.contains(obj) {
            Err(FlowError::IsOrphaned)
        } else 
        if &self.root == owner || &self.root == obj {
            Err(FlowError::RootDefect)
        } else 
        {
            if Some(owner.clone()) == self.node(obj)
                .map_or(None, |x| x.parent.clone()) 
            {
                Ok(())
            } else {
                {
                    let owner = self.node(owner).expect("checked");
                    if ! owner.children.contains(obj) {
                        return Err(FlowError::AbandonedChild)
                    }
                }
                let obj = self.node_mut(obj).expect("checked");
                obj.parent = Some(owner.clone());
                Ok(())

            }
        }
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
