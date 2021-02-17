#[allow(dead_code)]
#[allow(unused)]
pub mod identity;
pub mod time;
pub mod relation;
pub mod entry;
pub mod cube;
pub mod branch;

pub mod prelude {
    pub use crate::stockpile::*;
    pub use crate::stockpile::identity::*;
    pub use crate::stockpile::relation::*;
    pub use crate::stockpile::entry::*;
    pub use crate::stockpile::cube::*;
    pub use crate::stockpile::branch::*;
}


use identity::IdentityHash;
use crate::util::*;
use prelude::*;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Stockpile {
    pub branch: Branch
}

impl Stockpile {
    pub fn new() -> Self {
        Self {
            branch: Branch::new()
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


