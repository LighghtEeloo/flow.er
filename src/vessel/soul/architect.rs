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

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FlowLink<Id> {
    pub target: Option<Id>,
    pub dir: Direction,
    /// May not be used
    pub idx: FlowLinkIndex
}
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FlowLinkIndex {
    Head,
    Tail,
    Index(usize)
}
impl FlowLinkIndex {
    pub fn insert_with<Id: Identity>(&self, node: &mut FlowNode<Id>, obj: Id) {
        match self {
            FlowLinkIndex::Head => node.descendant.insert(0, obj),
            FlowLinkIndex::Tail => node.descendant.push(obj),
            FlowLinkIndex::Index(i) => node.descendant.insert(*i, obj)
        }
    }
}
impl Default for FlowLinkIndex {
    fn default() -> Self {
        FlowLinkIndex::Tail
    }
}

pub trait Architect<Id>: Debug
where Id: IdentityBase
{
    /// add new if not-found; return error if exist.
    fn add(&mut self, obj: &Id) -> Result<&Id, Critic>;
    ///  link two nodes as Ascend / Descend
    fn link(&mut self, obj: &Id, flow_link: FlowLink<Id>) -> Result<&Id, Critic>;
    /// del node; automatically de-link all the surrounding links.
    fn del(&mut self, obj: &Id) -> Result<(), Critic>;
}
