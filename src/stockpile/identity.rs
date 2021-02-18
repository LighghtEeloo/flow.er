use crate::util::*;
use crate::stockpile::time::*;

use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};


pub trait IdentityHash: Hash + Copy {
    fn from_time(v: &impl TimeRep) -> Self;
}

pub trait Identity: IdentityHash + PartialEq + Eq + Clone + Debug + Serialize + Deserialize<'static> {
    fn new_stamped() -> Self {
        let stamp = TimeStamp::created();
        Self::from_time(&stamp.data)
    }
}

const LEN: usize = 6;

#[derive(Clone, Copy, Hash, PartialEq, Eq, Deserialize, Serialize)]
pub struct EntryId (u64);

impl Debug for EntryId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> { 
        let hash: String = format!("{:x}", self.0).as_str().chars().take(LEN).collect();
        write!(f, "[[{}]]", hash) 
    }
}

impl IdentityHash for EntryId {
    fn from_time(v: &impl TimeRep) -> Self {
        let time = v.flatten()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_nanos();
        let mut s = DefaultHasher::new();
        time.hash(&mut s);
        EntryId (s.finish())
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
    fn from_time(v: &impl TimeRep) -> Self {
        let time = v.flatten()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_nanos();
        let mut s = DefaultHasher::new();
        time.hash(&mut s);
        CubeId (s.finish())
    }
}

impl Identity for CubeId {}
