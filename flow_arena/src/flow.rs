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
/// C. Devote & Decay
/// 
/// devote: mounts an orphaned node to a non-root node, and unmounts it from root.
/// 
/// decay: deletes all the links of a node and mounts it only to root.
/// 
/// With the devote and decay functions, now the flow can express a tree structure.
/// 
/// D. Link
/// 
/// link: randomly mounts an *unorphaned* node under a non-root node; the parent of the node will not be changed.
/// 
/// With link, now the flow can finally express a semi-graph structure with restrictions.
/// 
pub trait Flow {
    type Id: Default + Hash + Eq + Clone;
    type Node: Default;
    /// ensures root and returns it; no check
    fn root(&mut self) -> &mut Self::Node;
    /// no check
    fn node(&self, obj: &Self::Id) -> Option<&Self::Node>;
    /// inserts a node and mounts it to the root; returns err if id exists.
    fn grow(&mut self, obj: Self::Node) -> Result<(), ()>;
    /// mounts an orphaned node to a non-root node as the nth child, and unmounts it from root
    ///
    /// err if:
    /// 1. obj not mounted under root 
    /// 2. des is root 
    /// 3. nth > len
    /// 4. no obj / des
    fn devote(&mut self, obj: &Self::Id, des: &Self::Id, nth: usize) -> Result<(), ()>;
    fn devote_push(&mut self, obj: &Self::Id, des: &Self::Id) -> Result<(), ()>;
    /// randomly mounts an *unorphaned* node under a non-root node; the parent of the node will not be changed
    fn link(&mut self, obj: &Self::Id, des: &Self::Id, nth: usize) -> Result<(), ()>;
    fn link_push(&mut self, obj: &Self::Id, des: &Self::Id) -> Result<(), ()>;
    // /// insert a flow and devote to a node; err on id collision
    // fn merge_flow(&mut self, flow: Self, des: &Self::Id, nth: usize) -> Result<(), ()>;
    // fn merge_flow_push(&mut self, flow: Self, des: &Self::Id) -> Result<(), ()>;
    /// deletes all the links of a node and mounts it only to root
    fn decay(&mut self, obj: &Self::Id) -> Result<(), ()>;
    /// removes a node mounted to the root and unmounts it; returns err if id not found under root
    fn erase(&mut self, obj: &Self::Id) -> Result<(), ()>;
}


