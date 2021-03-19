use crate::util::*;
use super::prelude::*;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct FlowNode<Id> 
where Id: Identity
{
    pub descendant: Vec<Id>,
    /// owner:  
    /// Some(id) if there is one;  
    /// None if roots.
    pub owner: Option<Id>,
}
impl<Id> FlowNode<Id> 
where Id: Identity
{
    pub fn from_owner(elderly: Option<Id>) -> Self {
        Self {
            descendant: Vec::new(),
            owner: elderly,
        }
    }
}
impl<Id> Default for FlowNode<Id> 
where Id: Identity
{
    fn default() -> Self {
        Self::from_owner(None)
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct FlowLink<Id> {
    pub target: Option<Id>,
    pub dir: Direction,
    /// May not be used
    pub idx: FlowLinkIndex
}
impl<Id> FlowLink<Id> {
    fn new_descend(target: Id, idx: FlowLinkIndex) -> Self {
        Self {
            target: Some(target),
            dir: Direction::Descend,
            idx, 
        }
    }
    pub fn new_descend_head(target: Id) -> Self {
        Self::new_descend(target, FlowLinkIndex::Head)
    }
    pub fn new_descend_tail(target: Id) -> Self {
        Self::new_descend(target, FlowLinkIndex::Tail)
    }
    pub fn new_descend_index(target: Id, idx: usize) -> Self {
        Self::new_descend(target, FlowLinkIndex::Index(idx))
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FlowLinkIndex {
    Head,
    Tail,
    Index(usize)
}
impl FlowLinkIndex {
    pub fn inserted_to<Id: Identity>(&self, vec: &mut Vec<Id>, obj: Id) {
        match self {
            FlowLinkIndex::Head => vec.insert(0, obj),
            FlowLinkIndex::Tail => vec.push(obj),
            FlowLinkIndex::Index(i) => vec.insert(*i, obj)
        }
    }
    pub fn new_index(idx: usize) -> Self {
        FlowLinkIndex::Index(idx)
    }
    pub fn new_head() -> Self {
        FlowLinkIndex::Head
    }
    pub fn new_tail() -> Self {
        FlowLinkIndex::Tail
    }
}
impl Default for FlowLinkIndex {
    fn default() -> Self {
        FlowLinkIndex::Tail
    }
}

pub trait Architect<Id>: Debug
where Id: Identity
{
    /// add new if not-found; return error if exist.
    fn add(&mut self, obj: Id) -> Result<Id, Critic>;
    ///  link two nodes as Ascend / Descend
    fn link(&mut self, obj: Id, flow_link: FlowLink<Id>) -> Result<Id, Critic>;
    /// del node; automatically de-link all the surrounding links.
    fn del(&mut self, obj: Id) -> Result<(), Critic>;
}
