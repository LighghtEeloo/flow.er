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
/// a. tiptoe: out-of-nothing growth.  
/// 
/// b. chain: linked growth which transforms the RelationModel.
pub trait Chain<Id>: Grow<Id> 
where Id: IdentityHash
{
    /// tiptoe: out-of-nothing growth.
    fn tiptoe(&mut self, id: Id);
    /// chain: linked growth which transforms the RelationModel.
    fn chain(&mut self, new_comer: Id, host: Id);
}

/// Erase removes by id.
pub trait Erase<Id>: Grow<Id> 
where Id: IdentityHash
{
    fn erase(&mut self, id: Id);
}


