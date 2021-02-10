use crate::util::*;
use crate::cube::prelude::*;

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

#[derive(Debug)]
pub enum Direction {
    Up,
    Down,
}

pub trait RelationModel: Clone + fmt::Debug + Deserialize<'static> + Serialize + Sized {
    fn add(&mut self, target: EntryId, des: Option<EntryId>);
    fn del(&mut self, target: EntryId);
    fn focus(&mut self, target: EntryId);
    fn current(&self) -> Option<EntryId>;
    fn wander(&mut self, dir: Direction, fixed: bool);
    fn clear(&mut self);
}

// Linear

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct LinearModel {
    pub data: Vec<EntryId>,
    pub pos: Option<usize>,
    pub fix: Option<usize>,
}

impl LinearModel {
    pub fn new() -> Self {
        LinearModel::default()
    }
    fn locate(&self, target: EntryId) -> Option<usize> {
        self.data.iter().position(|&x| target == x)
    }
}

impl RelationModel for LinearModel {
    fn add(&mut self, target: EntryId, des: Option<EntryId>) {
        let pos = 
            match des {
                Some(des) => {
                    if let Some(pos) = self.locate(des) {
                        pos + 1
                    } else {
                        self.data.len()
                    }
                }
                None => 0
            };
        self.data.insert(pos, target);
        self.pos = Some(pos);
        self.fix = None;
    }
    fn del(&mut self, target: EntryId) {
        // Debug..
        LOG!("Deleting: {:?}", self.pos);
        match self.locate(target) {
            Some(pos) => {
                self.data.remove(pos);
                self.pos = None;
            }
            None => ()
        };
    }
    fn current(&self) -> Option<EntryId> {
        match self.pos {
            Some(pos) => Some(self.data[pos]),
            None => None
        }
    }
    fn focus(&mut self, target: EntryId) {
        self.pos = self.locate(target)
    }
    fn wander(&mut self, dir: Direction, fixed: bool) {
        if self.data.len() == 0 { return }
        let delta: isize = match dir {
            Direction::Up => -1,
            Direction::Down => 1
        };
        if fixed {
            // Todo: around fix.
            if let None = self.fix {
                self.fix = self.pos;
            }
            if let Some(center) = self.fix {
                self.pos = match self.data.len() {
                    0 => None,
                    _ => Some((center as isize + delta) as usize % self.data.len())
                };
                // let range: Vec<usize> = 
                //     vec![center - 1, center, center + 1].into_iter()
                //     .filter(|&x| x < self.data.len()).collect();
            };
        } else {
            self.fix = None;
            self.pos = match self.pos {
                Some(pos) => {
                    let pos = pos as isize + delta;
                    if pos < 0 {
                        None
                    } else if pos < self.data.len() as isize {
                        Some(pos as usize)
                    } else {
                        Some(self.data.len() - 1)
                    }
                }
                None => {
                    match dir {
                        Direction::Up => None,
                        Direction::Down => Some(0)
                    }
                }
            };
        }


    }
    fn clear(&mut self) {
        mem::take(self);
    }
}

impl Default for LinearModel {
    fn default() -> Self {
        Self {
            data: Vec::new(),
            pos: None,
            fix: None,
        }
    }
}

// make_model

// fn make_model(model_type: RelationModelType) -> Box<RelationModel> {
//     use RelationModelType::*;
//     match model_type {
//         Linear => Box::new(LinearModel::new())
//     }
// }
