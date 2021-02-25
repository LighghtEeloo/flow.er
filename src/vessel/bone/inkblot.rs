use crate::util::*;
use crate::yew_util::*;
use super::prelude::*;


/// Inkblot
/// Shows bubble of the entity.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Inkblot<Id>
where Id: Identity
{
    pub target: Id,
    // #[serde(skip)]
    // refs: HashMap<Id, NodeRef>,
    pub pos: Option<Id>,
}

impl<Id> Inkblot<Id>
where Id: Identity 
{
    pub fn from_flow(flow: &Flow<Id>, target: &Id) -> Self {
        Self {
            target: *target,
            pos: None,
        }
    }
    pub fn from_flow_boxed(flow: &Flow<Id>, target: &Id) -> Box<Self> {
        Box::new(Self::from_flow(flow, target))
    }
}


// // Artist

impl<Id> Artist<Id> for Inkblot<Id> where Id: Identity {}


// Animator

impl<Id> Animator<Id> for Inkblot<Id> 
where Id: Identity
{
    fn compute(&mut self) { 
        todo!() 
    }
    fn illustrate(&self) -> Html { 
        todo!() 
    }
}


// Dancer

impl<Id> Dancer<Id> for Inkblot<Id>
where Id: Identity
{
    fn check(&self, obj: &Id) -> Result<Id, Critic> {
        Ok(*obj)
    }
    fn current(&self) -> Option<Id> {
        self.pos.clone()
    }
    fn focus(&mut self, obj: Id) {
        todo!()
    }
    fn wander(&mut self, dir: Direction, fixed: bool) {
        todo!()
    }
}

