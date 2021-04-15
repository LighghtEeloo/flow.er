use std::{collections::HashSet, hash::Hash};

pub trait FlowNode {
    type Id: Clone;
    fn id(&self) -> &Self::Id;
    fn parent(&self) -> Option<Self::Id>;
    fn children(&self) -> Vec<&Self::Id>;
}

pub trait FlowBase {
    type Id: Default + Hash + Eq + Clone;
    type Node: Default + FlowNode;
    /// ensures root and returns it; no check
    fn orphan(&self) -> Vec<Self::Id>;
    /// no check hereafter
    fn contains_node(&self, obj: &Self::Id) -> bool {
        self.node(obj).is_some()
    }
    fn node(&self, obj: &Self::Id) -> Option<&Self::Node>;
    fn node_mut(&mut self, obj: &Self::Id) -> Option<&mut Self::Node>;
    fn parent(&self, obj: &Self::Id) -> Option<Self::Id>;
    fn children(&self, obj: &Self::Id) -> Vec<Self::Id>;
    /// returns parent's children
    fn friends(&self, obj: &Self::Id) -> Vec<Self::Id> {
        self.parent(obj).map_or(Vec::new(), |obj| {
            self.children(&obj)
        })
    }
    /// returns owned children
    fn children_owned(&self, obj: &Self::Id) -> Vec<Self::Id> {
        self.children(obj).into_iter().filter_map(|id| {
            if self.parent(&id) == Some(obj.clone()) {
                Some(id)
            } else {
                None
            }
        }).collect()
    }
    fn node_offspring_set(&self, obj: &Self::Id) -> HashSet<Self::Id>;
    fn node_ownership_set(&self, obj: &Self::Id) -> HashSet<Self::Id>;
}

pub trait FlowLink: FlowBase {
    /// randomly ensures the link of a node to another
    fn link(&mut self, obj: &Self::Id, owner: &Self::Id, nth: usize) -> Result<(), FlowError>;
    fn link_push(&mut self, obj: &Self::Id, owner: &Self::Id) -> Result<(), FlowError>;
    /// detaches a node from a non-owner link
    fn detach(&mut self, obj: &Self::Id, owner: &Self::Id) -> Result<(), FlowError>;
}

pub trait FlowMaid: FlowBase + FlowLink {
    /// inserts a node; returns err if id exists.
    fn grow(&mut self, obj: Self::Node) -> Result<Self::Id, FlowError>;
    /// appoints and ensures an owner; also links
    fn devote(&mut self, obj: &Self::Id, owner: &Self::Id, nth: usize) -> Result<(), FlowError>;
    fn devote_push(&mut self, obj: &Self::Id, owner: &Self::Id) -> Result<(), FlowError>;
    /// removes ownership; also detaches
    fn decay(&mut self, obj: &Self::Id) -> Result<(), FlowError>;
    /// removes a node; returns err if id not found under root
    fn erase(&mut self, obj: &Self::Id) -> Result<(), FlowError>;
}

pub trait FlowDock: FlowMaid + Sized {
    /// adds all the nodes in another flow to self and mounts all orphan nodes to the designated node
    ///
    /// Err if:
    /// 1. Owner not found.
    /// 2. Node exists in current flow.
    fn dock(&mut self, owner: &Self::Id, flow: Self) -> Result<(), FlowError>;
    /// moves all the nodes under the designated node out of the current flow and unmounts them
    /// 
    /// Err if:
    /// 1. Obj not found.
    /// 2. Node linked by other nodes.
    fn undock(&mut self, obj: &Self::Id) -> Result<Self, FlowError>;
    /// clones all the nodes under the designated node and unmounts them
    /// 
    /// Err if:
    /// 1. Obj not found.
    fn snap(&self, obj: &Self::Id) -> Result<Self, FlowError>;
}

pub enum Direction {
    Forward,
    Backward,
    Ascend,
    Descent,
}

pub trait FlowShift: FlowBase {
    /// returns the obj in the corresponding relative position
    fn shuttle(&self, obj: &Self::Id, dir: Direction) -> Result<Self::Id, FlowError>;
    /// alters the node position by the corresponding relative position, within a single node
    fn migrate(&self, obj: &Self::Id, dir: Direction) -> Result<(), FlowError>;
    /// alters the node position by the corresponding relative position, iteratively within the flow
    fn migrate_iter(&self, obj: &Self::Id, dir: Direction) -> Result<(), FlowError>;
}

#[derive(Debug)]
pub enum FlowError {
    NotExistObj,
    NotExistOwner,
    InValidLen,
    ExistGrow,
    OwnerDetach,
    /// certain operations requires node to be orphaned
    NotOrphaned,
    /// certain operations requires node to be unorphaned
    IsOrphaned,
    
    NodeIdNotMatch,
    NotExistParent,
    NotExistChild,
    /// potential parent doesn't have the child
    AbandonedChild,
}

/// Flow: the underlying trait for flow.er.
/// 
/// Let's start with some key concepts.
/// 
/// A. Arena
/// 
/// An arena is a typical data structure which has:
/// 1. A map / vec to store the data.
/// 2. A relationship graph which only tracks after the keys / indices.
/// 
/// FlowArena implements an arena-like data structure, but it has integrated the data map and the relationship graph, since both of them require an Id to visit. 
/// 
// Todo..
pub trait Flow: FlowBase + FlowLink + FlowMaid + FlowDock + FlowShift {
    fn check(&self) -> Result<(), (FlowError, String)>;
        /// panics if anything went wrong. Iff in debug state.
    fn check_assert(&self) {
        if cfg!(debug_assertions) {
            if let Err((err, current)) = self.check() {
                panic!("{:?}{}", err, current)
            }   
        } 
    } 
}
