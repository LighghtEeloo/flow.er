
pub trait Flow {
    type Id;
    type Node;
    /// ensures root and returns it; no check
    fn root(&mut self) -> &mut Self::Node;
    /// no check
    fn node(&self, obj: &Self::Id) -> Option<&Self::Node>;
    /// inserts obj to node_map; err if exist
    fn grow(&mut self, obj: Self::Node) -> Result<(), ()>;
    /// link obj as a child of des at the nth place; 
    /// automatically removes it from root.
    /// err if nth > len or no obj / des
    fn devote(&mut self, obj: &Self::Id, des: &Self::Id, nth: usize) -> Result<(), ()>;
    fn devote_push(&mut self, obj: &Self::Id, des: &Self::Id) -> Result<(), ()>;
    // /// insert a flow and devote to a node; err on id collision
    // fn merge_flow(&mut self, flow: Self, des: &Self::Id, nth: usize) -> Result<(), ()>;
    // fn merge_flow_push(&mut self, flow: Self, des: &Self::Id) -> Result<(), ()>;
    /// decay the node by removing only its connection between 
    /// its parent and itself; mounts the node to root.
    fn decay(&mut self, obj: &Self::Id) -> Result<(), ()>;
    /// cuts all the links (and mounts only to root), but doesn't remove.
    fn purge(&mut self, obj: &Self::Id) -> Result<(), ()>;
    /// removes from node_map and purges.
    fn erase(&mut self, obj: &Self::Id) -> Result<(), ()>;
}


