use crate::prelude::*;
use crate::cube::*;

#[derive(Clone, Debug, Deserialize, Serialize, EnumString, EnumVariantNames, EnumIter, EnumProperty, ToString)]
pub enum RelationModel {
    Linear(Vec<EntryId>),
    Graph,
    // Todo: Garph RelationModel.
}

use RelationModel::*;

impl RelationModel {
    pub fn clear(&mut self) {
        match self {
            Linear(_) => {
                mem::swap(self, &mut Linear(vec!()))
            }
            Graph => {
                unimplemented!()
            }
        }
    }
}

impl Default for RelationModel {
    fn default() -> Self { 
        Self::Linear(vec!())
    }
}
