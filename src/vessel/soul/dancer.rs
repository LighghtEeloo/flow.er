use crate::util::*;
use super::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Direction {
    Ascend,
    Descend,
    Stay
}

impl Direction {
    pub fn translate(&self) -> isize {
        match self {
            Direction::Ascend => -1,
            Direction::Stay => 0,
            Direction::Descend => 1
        }
    }
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub enum FixState<Id> {
    Deactivated,
    Descendant(Vec<Id>, isize),
    Owner(Vec<Id>, isize),
}
impl<Id> FixState<Id> 
where Id: Identity
{
    pub fn to_id(&self) -> Option<Id> {
        use FixState::*;
        match self {
            Deactivated => None,
            Owner(vec, idx) => vec.get(*idx as usize).cloned(),
            Descendant(vec, idx) => vec.get(*idx as usize).cloned(),
        }
    }
    pub fn deactivate(&mut self) {
        mem::swap(&mut FixState::Deactivated, self);
    }
}


// pub struct RelativePath<Id> 
// where Id: Identity
// {
//     path: Vec<(Id, Direction)>
// }


pub trait Dancer<Id>: Debug
where Id: IdentityBase
{
    fn check(&self, obj: &Id) -> Result<Id, Critic>;
    /// Return the current pos in Dancer.
    fn current(&self) -> Option<Id>;
    /// Update pos.
    fn focus(&mut self, obj: Id);
    fn wander(&mut self, dir: Direction, fixed: bool);
}
