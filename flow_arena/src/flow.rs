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
/// B. Root - a fake node
/// 
/// Root is a special node which always exists in the node map. Currently its Id is always Id::default.
/// 
/// The basic rule for any node mounted under root is uni-ownership - the root must be the only owner. This can be specified by the following:
/// 
/// B.1. Orphaned
/// 
/// If a node is only mounted to root, then the node is orphaned. A node mounted to root can't be "childed" by other nodes.
/// 
/// B.2. Grow & Erase
/// 
/// grow: inserts a node and mounts it to the root; returns err if id exists.
/// 
/// erase: removes a node mounted to the root and unmounts it; returns err if id not found under root.
/// 
/// To sum up, all the node "registration" processes are done via mounting and un-mounting to root.
/// 
/// C. Tree Ability: Devote & Decay
/// 
/// devote: mounts an orphaned node to a non-root node, and unmounts it from root.
/// 
/// decay: deletes all the links of a node and mounts it only to root.
/// 
/// With the devote and decay functions, now the flow can express a tree structure.
/// 
/// D. Graph Ability
/// 
/// D.1. Link & Detach
/// 
/// link: randomly mounts an *unorphaned* node under a non-root node; the parent of the node will not be changed.
/// 
/// detach: unmounts an *unorphaned* node from a non-root node; the parent of the node must still own the node.
/// 
/// With link and detach, now the flow can finally express a semi-graph structure with restrictions.
/// 
/// D.2. Defect
/// 
/// defect: switches an *unorphaned* node's owner to be another non-root node which already has access to the node.
/// 
/// E. Linkage Complier
/// 
/// Though the relationship is stored in the relationship graph, a linkage compiler is implemented \[in progress\] to translate it to a non-recrusive format.
/// 
/// Waiting for update...
pub trait Flow: FlowMap + FlowTree + FlowGraph {}

pub trait FlowMap {
    type Id: Default + Hash + Eq + Clone;
    type Node: Default;
    /// ensures root and returns it; no check
    fn root(&mut self) -> &mut Self::Node;
    /// no check
    fn node(&self, obj: &Self::Id) -> Option<&Self::Node>;
    /// no check
    fn node_mut(&mut self, obj: &Self::Id) -> Option<&mut Self::Node>;
    /// inserts a node and mounts it to the root; returns err if id exists.
    fn grow(&mut self, obj: Self::Node) -> Result<Self::Id, FlowError>;
    /// removes a node mounted to the root and unmounts it; returns err if id not found under root
    fn erase(&mut self, obj: &Self::Id) -> Result<Self::Node, FlowError>;
}


pub trait FlowTree: FlowMap {
    /// mounts an orphaned node to a non-root node as the nth child, and unmounts it from root
    ///
    /// err if:
    /// 1. obj not mounted under root 
    /// 2. owner is root 
    /// 3. nth > len
    /// 4. no obj / owner
    fn devote(&mut self, obj: &Self::Id, owner: &Self::Id, nth: usize) -> Result<(), FlowError>;
    fn devote_push(&mut self, obj: &Self::Id, owner: &Self::Id) -> Result<(), FlowError>;
    /// deletes all the links of a node and mounts it only to root
    fn decay(&mut self, obj: &Self::Id) -> Result<(), FlowError>;
}


pub trait FlowGraph: FlowMap {
    /// randomly mounts an *unorphaned* node under a non-root node; the parent of the node will not be changed
    fn link(&mut self, obj: &Self::Id, owner: &Self::Id, nth: usize) -> Result<(), FlowError>;
    fn link_push(&mut self, obj: &Self::Id, owner: &Self::Id) -> Result<(), FlowError>;
    /// unmounts an *unorphaned* node from a non-root node; the parent of the node must still own the node
    fn detach(&self, obj: &Self::Id) -> Result<(), FlowError>;
    /// switches an *unorphaned* node's owner to be another non-root node which already has access to the node
    fn defect(&mut self, obj: &Self::Id, owner: &Self::Id) -> Result<(), FlowError>;
}

#[derive(Debug)]
pub enum FlowError {
    NoRoot,
    NotDefaultRoot,
    ParentedRoot,

    NotExistObj,
    NotExistOwner,
    InValidLen,
    ExistGrow,
    RootDevote,
    RootDecay,
    RootLink,
    /// certain operations requires node to be orphaned
    NotOrphaned,
    /// certain operations requires node to be unorphaned
    IsOrphaned,
    
    NodeIdUnmatch,
    /// root children must not be owned by other nodes
    NotStrictlyOrphaned,
    /// non-root nodes must have parent
    NoParent,
    NotExistParent,
    NotExistChild,
    /// potential parent doesn't have the child
    AbandonedChild,
}

