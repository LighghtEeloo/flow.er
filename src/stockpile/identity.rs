use crate::util::*;
use crate::stockpile::time::*;

use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};


pub trait IdentityHash: Hash + Copy {
    fn from_time(v: &impl TimeRep) -> Self;
}

pub trait Identity: IdentityHash + PartialEq + Eq + Clone + Debug + Serialize + Deserialize<'static> {}



#[derive(Clone, Copy, Hash, PartialEq, Eq, Deserialize, Serialize)]
pub struct EntryId (u64);

impl Debug for EntryId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> { 
        // write!(f, "[{}.{}]", self.0, self.1) 
        write!(f, "[[{}]]", self.0) 
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
        // format!("{:x}", s.finish())
        EntryId (s.finish())
        // let time = v.flatten()
        //     .duration_since(UNIX_EPOCH)
        //     .expect("Time went backwards");
        // EntryId (time.as_secs(), time.subsec_nanos())
    }
}

impl Identity for EntryId {}



#[derive(Clone, Copy, Hash, PartialEq, Eq, Deserialize, Serialize)]
pub struct CubeId (u64);

impl Debug for CubeId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> { 
        write!(f, "{{{}}}", self.0) 
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
