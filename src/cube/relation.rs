// use crate::prelude::*;
use crate::cube::*;

#[derive(Clone, Debug, Deserialize, Serialize, EnumString, EnumVariantNames, EnumIter, EnumProperty, ToString)]
pub enum RelationModel {
    Linear(Vec<EntryId>),
    Graph
    // Todo: Garph RelationModel.
}
