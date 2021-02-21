use crate::util::*;
use super::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Direction {
    Ascend,
    Descend,
    Stay
}

impl Direction {
    fn translate(&self) -> isize {
        match self {
            Direction::Ascend => -1,
            Direction::Stay => 0,
            Direction::Descend => 1
        }
    }
}

pub struct RelativePath<Id> 
where Id: Identity
{
    path: Vec<(Id, Direction)>
}


pub trait Dancer<Id>: Clone + Debug
where Id: Identity
{
    // fn add(&mut self, obj: Id, des: Option<Id>);
    // fn del(&mut self, obj: Id);
    // fn focus(&mut self, obj: Id);
    // fn current(&self) -> Option<Id>;
    // fn wander(&mut self, dir: Direction, fixed: bool);
    fn clear(&mut self);
}
