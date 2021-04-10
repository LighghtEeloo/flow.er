use std::hash::Hash;

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
pub trait Flow: FlowMap + FlowLink + FlowMaid {}

pub trait FlowMap {
    type Id: Default + Hash + Eq + Clone;
    type Node: Default;
    /// ensures root and returns it; no check
    fn orphan(&self) -> Vec<Self::Id>;
    /// no check
    fn node(&self, obj: &Self::Id) -> Option<&Self::Node>;
    /// no check
    fn node_mut(&mut self, obj: &Self::Id) -> Option<&mut Self::Node>;
    /// inserts a node; returns err if id exists.
    fn grow(&mut self, obj: Self::Node) -> Result<Self::Id, FlowError>;
}

pub trait FlowLink: FlowMap {
    /// randomly ensures the link of a node to another
    fn link(&mut self, obj: &Self::Id, owner: &Self::Id, nth: usize) -> Result<(), FlowError>;
    fn link_push(&mut self, obj: &Self::Id, owner: &Self::Id) -> Result<(), FlowError>;
    /// detaches a node from a non-owner link
    fn detach(&mut self, obj: &Self::Id, owner: &Self::Id) -> Result<(), FlowError>;
}

pub trait FlowMaid: FlowMap + FlowLink {
    /// appoints and ensures an owner; also links
    fn devote(&mut self, obj: &Self::Id, owner: &Self::Id, nth: usize) -> Result<(), FlowError>;
    fn devote_push(&mut self, obj: &Self::Id, owner: &Self::Id) -> Result<(), FlowError>;
    /// removes ownership; also detaches
    fn decay(&mut self, obj: &Self::Id) -> Result<(), FlowError>;
    /// removes a node; returns err if id not found under root
    fn erase(&mut self, obj: &Self::Id) -> Result<(), FlowError>;
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
    
    NodeIdUnmatch,
    NotExistParent,
    NotExistChild,
    /// potential parent doesn't have the child
    AbandonedChild,
}

