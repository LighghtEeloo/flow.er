pub mod relation;
pub mod entry;
pub mod time;
#[allow(dead_code)]
#[allow(unused)]

use crate::util::*;
pub use relation::*;
pub use entry::*;

// pub use error::{Result, Error};

pub mod prelude {
    pub use crate::cube::*;
    pub use crate::cube::relation::*;
}



#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Cube {
    pub name: String,
    pub locked: bool,
    pub entries: HashMap<EntryId, Entry>,
    // Todo: polymorphism
    pub relation: LinearModel<EntryId>,
}

impl Cube {
    pub fn new() -> Self {
        Cube {
            name: String::new(),
            locked: false,
            entries: HashMap::new(),
            relation: LinearModel::new()
        }
    }
    pub fn empty(&self) -> bool {
        self.entries.len() == 0
    }
    pub fn get(&self, id: EntryId) -> &Entry {
        self.entries.get(&id).unwrap()
    }
}

/// Grow aims at any object that grows anonymously. 
/// Chain can bundle the change to the RelationModel
pub trait Grow<Id: IdentityHash> {
    fn grow(&mut self) -> Id;
}
impl Grow<EntryId> for Cube {
    fn grow(&mut self) -> EntryId {
        let entry = Entry::new();
        let id = entry.id();
        self.entries.insert(id, entry);
        id
    }
}

/// Chain 
/// 1. tiptoe: out-of-nothing growth.
/// 2. chain: linked growth which transforms the RelationModel.
pub trait Chain<Id: IdentityHash>: Grow<Id> {
    fn tiptoe(&mut self, id: Id);
    fn chain(&mut self, new_comer: Id, host: Id);
}
impl Chain<EntryId> for Cube {
    fn tiptoe(&mut self, id: EntryId) {
        self.relation.add(id, None);
    }
    fn chain(&mut self, new_comer: EntryId, host: EntryId) {
        self.relation.add(new_comer, Some(host))
    }
}

pub trait Erase<Id: IdentityHash>: Grow<Id> {
    fn erase(&mut self, id: Id);
}

impl Erase<EntryId> for Cube {
    fn erase(&mut self, id: EntryId) {
        if let Some(_) = self.entries.remove(&id) {
            self.relation.del(id);
        }
    }
}
