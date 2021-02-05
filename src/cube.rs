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
}



#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Cube {
    pub name: String,
    pub locked: bool,
    pub entries: HashMap<EntryId, Entry>,
    pub relation: RelationModel,
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
        use RelationModel::*;
        match &mut self.relation {
            Linear(vec) => {
                vec.model.insert(0, id);
                vec.idx = 0;
            }
            Tree => (),
            Graph => ()
        }
    }
    fn chain(&mut self, new_comer: EntryId, host: EntryId) {
        use RelationModel::*;
        match &mut self.relation {
            Linear(vec) => {
                let pos = vec.model.into_iter().position(|x| x.clone() == host);
                if let Some(p) = pos {
                    vec.model.insert(p+1, new_comer);
                    vec.idx = p+1;
                } else {
                    vec.model.push(new_comer);
                    vec.idx = vec.model.len() - 1;
                }
            }
            Tree => (),
            Graph => ()
        }
    }
}



