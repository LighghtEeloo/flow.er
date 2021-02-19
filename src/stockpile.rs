#[allow(dead_code)]
#[allow(unused)]
pub mod identity;
pub mod time;
pub mod tag_set;
pub mod relation;
pub mod entry;
pub mod cube;
pub mod branch;

pub mod prelude {
    pub use super::*;
    pub use super::time::*;
    pub use super::identity::*;
    pub use super::tag_set::*;
    pub use super::relation::*;
    pub use super::entry::*;
    pub use super::cube::*;
    pub use super::branch::*;
}


use identity::IdentityHash;
use crate::util::*;
use prelude::*;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Stockpile {
    pub branch: Branch,
    pub history: Vec<TimeStamp<Branch>>,
    pub editor_info: EditorInfo,
}

impl Default for Stockpile {
    fn default() -> Self {
        Self {
            branch: Branch::new(),
            history: Vec::new(),
            editor_info: EditorInfo::new_none(),
        }
    }
}

impl Stockpile {
    pub fn new() -> Self {
        Self::default()
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct EditorInfo {
    pub cube_id: Option<CubeId>,
}

impl EditorInfo {
    pub fn new_some(cube_id: CubeId) -> Self {
        Self {
            cube_id: Some(cube_id)
        }
    }
    pub fn new_none() -> Self {
        Self {
            cube_id: None,
        }
    }
}

/// Grow describes any object that grows anonymously. 
/// 
/// Chain can bundle the change to the RelationModel
pub trait Grow<Id> 
where Id: IdentityHash
{
    fn grow(&mut self) -> Id;
}

/// Chain has two member methods.  
/// 
/// a. tiptoe: out-of-nothing growth, like orphan.  
/// 
/// b. chain: linked growth which transforms the RelationModel; 
/// des is None if replacing root, Some(id) if following node.
pub trait Chain<Id>: Grow<Id> 
where Id: IdentityHash
{
    /// tiptoe: out-of-nothing growth, like orphan.
    fn tiptoe(&mut self, obj: Id);
    /// chain: linked growth which transforms the RelationModel; 
    /// des is None if replacing root, Some(id) if following node.
    fn chain(&mut self, obj: Id, des: Option<Id>);
}

/// Erase removes by id.
pub trait Erase<Id>: Grow<Id> 
where Id: IdentityHash
{
    fn erase(&mut self, id: Id);
}


