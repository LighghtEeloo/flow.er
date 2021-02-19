use crate::util::*;
use crate::stockpile::time::*;

use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};


const LEN: usize = 6;

pub trait IdentityHash: Hash + Copy {
    fn from_u64(v: u64) -> Self;
    fn from_time(v: &impl TimeRep) -> Self {
        let mut s = DefaultHasher::new();
        v.flatten().hash(&mut s);
        Self::from_u64(s.finish())
    }
}

/// For id.
pub trait Identity: IdentityHash + PartialEq + Eq + Clone + Debug + Serialize + Deserialize<'static> {
    fn new_stamped() -> Self {
        let stamp = TimeStamp::<()>::new();
        Self::from_time(&stamp)
    }
}

/// For producing obj with id.
pub trait IdentityProduct<Id: Identity>: Default {
    fn new() -> Self {
        Self::with_id(Id::new_stamped())
    }
    fn with_id(id: Id) -> Self;
    fn id(&self) -> Id;
}

/// Collection: get & set. For hashmap users of obj with id.
pub trait IdentityMap<Id: Identity, Value: Default> {
    /// is_empty
    fn is_empty(&self) -> bool;

    /// get Value including dummy.
    fn get_cloned(&self, id: Id) -> Value;

    /// get &Value; insert if not included.
    fn get_update(&mut self, id: Id) -> &Value;
    
    /// get &mut Value; insert if not included.
    fn get_mut(&mut self, id: Id) -> &mut Value;

    /// set pair if id exists; Err((Id, Value)) if not exist.
    fn set(&mut self, id: Id, value: Value) -> Result<(), (Id, Value)>;
}

#[derive(Clone, Copy, Hash, PartialEq, Eq, Deserialize, Serialize)]
pub struct EntryId (u64);

impl Debug for EntryId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> { 
        let hash: String = format!("{:x}", self.0).as_str().chars().take(LEN).collect();
        write!(f, "[[{}]]", hash) 
    }
}

impl IdentityHash for EntryId {
    fn from_u64(v: u64) -> Self {
        EntryId (v)
    }
}
impl Identity for EntryId {}



#[derive(Clone, Copy, Hash, PartialEq, Eq, Deserialize, Serialize)]
pub struct CubeId (u64);

impl Debug for CubeId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> { 
        let hash: String = format!("{:x}", self.0).as_str().chars().take(LEN).collect();
        write!(f, "{{{{{}}}}}", hash)
    }
}

impl IdentityHash for CubeId {
    fn from_u64(v: u64) -> Self {
        CubeId (v)
    }
}
impl Identity for CubeId {}
