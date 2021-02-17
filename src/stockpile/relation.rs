use crate::util::*;
use crate::stockpile::prelude::*;

mod linear;
mod flow;

pub use linear::LinearModel;
pub use flow::FlowModel;


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

pub trait RelationModel<Id>: Clone + Debug + Deserialize<'static> + Serialize {
    fn add(&mut self, obj: Id, des: Option<Id>);
    fn del(&mut self, obj: Id);
    fn focus(&mut self, obj: Id);
    fn current(&self) -> Option<Id>;
    fn wander(&mut self, dir: Direction, fixed: bool);
    fn clear(&mut self);
}


// make_model

// fn make_model(model_type: RelationModelType) -> Box<RelationModel> {
//     use RelationModelType::*;
//     match model_type {
//         Linear => Box::new(LinearModel::new())
//     }
// }
