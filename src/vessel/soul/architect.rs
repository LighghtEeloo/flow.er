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
pub struct FlowAdd<Id> {
    pub target: Option<Id>,
    pub dir: Direction,
    /// May not be used
    pub idx: FlowAddIndex
}
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FlowAddIndex {
    Head,
    Tail,
    Index(usize)
}

impl FlowAddIndex {
    pub fn insert_with<Id: Identity>(&self, node: &mut FlowNode<Id>, obj: Id) {
        match self {
            FlowAddIndex::Head => node.descendant.insert(0, obj),
            FlowAddIndex::Tail => node.descendant.push(obj),
            FlowAddIndex::Index(i) => node.descendant.insert(*i, obj)
        }
    }
}

pub trait Architect<Id>: Debug
where Id: IdentityBase
{
    fn add(&mut self, obj: &Id, flow_add: FlowAdd<Id>) -> Result<Id, FlowNodeExistError>;
}
