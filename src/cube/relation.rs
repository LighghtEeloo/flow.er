use crate::prelude::*;
use crate::cube::*;

#[derive(Clone, Debug, Deserialize, Serialize, EnumString, EnumVariantNames, EnumIter, EnumProperty, ToString)]
pub enum RelationModel {
    Linear(LinearModel),
    Tree,
    // Todo: Tree RelationModel.
    Graph,
    // Todo: Garph RelationModel.
}

use RelationModel::*;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct LinearModel {
    pub model: Vec<EntryId>,
    pub idx: usize,
}

impl LinearModel {
    pub fn new() -> RelationModel {
        Linear(LinearModel::default())
    }
}

impl Default for LinearModel {
    fn default() -> Self {
        Self {
            model: Vec::new(),
            idx: 0
        }
    }
}

impl RelationModel {
    pub fn type_str(&self) -> &str {
        match self {
            Linear(_) => "Linear",
            Tree => "Tree",
            Graph => "Graph",
        }
    }
    pub fn clear(&mut self) {
        let new_model = match self {
            Linear(_) => {
                LinearModel::new()
            }
            Tree => {
                unimplemented!()
            }
            Graph => {
                unimplemented!()
            }
        };
        mem::swap(self, &mut new_model)
    }
    pub fn current(&self) -> EntryId {
        match self {
            Linear(linear) => {
                linear.model[linear.idx]
            }
            Tree => {
                unimplemented!()
            }
            Graph => {
                unimplemented!()
            }
        }
    }
    pub fn up(&mut self) {
        let new_nodel = match self {
            Linear(mut linear) => {
                if (linear.idx > 0) {
                    linear.idx -= 1;
                }
                // Debug..
                LOG!("Up: {:?}", linear);
                Linear(linear)
            }
            Tree => {
                unimplemented!()
            }
            Graph => {
                unimplemented!()
            }
        };
        mem::swap(self, &mut new_nodel)
    }
    pub fn down(&mut self) {
        let new_nodel = match self {
            Linear(mut linear) => {
                if (linear.idx < linear.model.len() - 1) {
                    linear.idx += 1;
                }
                // Debug..
                LOG!("Down: {:?}", linear);
                Linear(linear)
            }
            Tree => {
                unimplemented!()
            }
            Graph => {
                unimplemented!()
            }
        };
        mem::swap(self, &mut new_nodel)
    }
}

impl Default for RelationModel {
    fn default() -> Self { 
        Self::Linear(vec!())
    }
}
