use crate::util::*;
use crate::cube::prelude::*;

mod linear;

pub use linear::LinearModel;


// #[derive(Clone, Debug, Deserialize, Serialize)]
// pub enum RelationModelType {
//     Linear,
//     Tree,
//     Graph,
// }

// impl RelationModelType {
//     pub fn type_str(&self) -> &str {
//         match self {
//             Linear => "Linear",
//             Tree => "Tree",
//             Graph => "Graph",
//         }
//     }
// }

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Direction {
    Up,
    Down,
    Stay
}

impl Direction {
    fn translate(&self) -> isize {
        match self {
            Direction::Up => -1,
            Direction::Stay => 0,
            Direction::Down => 1
        }
    }
}

pub trait RelationModel: Clone + fmt::Debug + Deserialize<'static> + Serialize + Sized {
    fn add(&mut self, target: EntryId, des: Option<EntryId>);
    fn del(&mut self, target: EntryId);
    fn focus(&mut self, target: EntryId);
    fn current(&self) -> Option<EntryId>;
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
