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
    // /// [translate] the movement proposal
    // pub fn translate(&self, dir: Direction) -> isize {
    //     use FixState::*;
    //     use Direction::*;
    //     0
    // }
    // /// perform the self [shift] with delta
    // pub fn shift_delta(&mut self, delta: isize) {
    //     use FixState::*;
    //     let pos: isize = match self {
    //         Relative(x) => x.clone(),
    //         _ => 0
    //     };
    //     let mut pos_new = pos + delta;
    //     pos_new += 1;
    //     pos_new = ((pos_new % 3) + 3) % 3; // modulus
    //     pos_new -= 1;
    //     let mut next = Relative(pos_new);
    //     mem::swap(&mut next, self);
    // }
    // pub fn activate(&mut self) {
    //     use FixState::*;
    //     let mut next = match self {
    //         Deactivated => Relative(0),
    //         _ => self.clone()
    //     };
    //     mem::swap(&mut next, self);
    // }
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


pub trait Dancer<Id>: Clone + Debug
where Id: Identity
{
    fn check(&self, obj: &Id) -> Result<Id, FlowNodeNotFoundError>;
    /// Return the current pos in Dancer.
    fn current(&self) -> Option<Id>;
    /// Update pos.
    fn focus(&mut self, obj: Id);
    fn wander(&mut self, dir: Direction, fixed: bool);
}
