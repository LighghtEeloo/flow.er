/// identity.rs contains:
/// 1. trait IdentityHash
/// 2. trait Identity += IdentityHash
/// 3. trait IdentityProduct
/// 4. trait IdentityMap

use crate::util::*;
use super::time::*;

use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};


const LEN: usize = 6;


/// For id.
pub trait Identity: Hash + Copy + PartialEq + Eq + Clone + Debug {
    fn from_u64(v: u64) -> Self;
    fn from_time(v: &impl TimeRep) -> Self {
        let mut s = DefaultHasher::new();
        v.flatten().hash(&mut s);
        Self::from_u64(s.finish())
    }
    fn new_stamped() -> Self {
        let stamp = TimeStamp::now();
        Self::from_time(&stamp)
    }
}

/// For producing obj with id.
pub trait IdentityProduct<Id: Identity>: Default + Clone + Debug {
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
pub struct EntityId (u64);

impl Debug for EntityId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> { 
        let hash: String = format!("{:x}", self.0).as_str().chars().take(LEN).collect();
        write!(f, "[[{}]]", hash) 
    }
}

impl Identity for EntityId {
    fn from_u64(v: u64) -> Self {
        EntityId (v)
    }
}



#[derive(Clone, Copy, Hash, PartialEq, Eq, Deserialize, Serialize)]
pub struct VesselId (u64);

impl Debug for VesselId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> { 
        let hash: String = format!("{:x}", self.0).as_str().chars().take(LEN).collect();
        write!(f, "{{{{{}}}}}", hash)
    }
}

impl Identity for VesselId {
    fn from_u64(v: u64) -> Self {
        VesselId (v)
    }
}
