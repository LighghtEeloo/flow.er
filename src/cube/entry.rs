use crate::util::*;
use crate::cube::time::*;

use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};


// Entry Area

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Entry {
    dry: EntryDry,
    timestamps: Vec<TimeStamp>,
    // Todo: Add positional info.
    // position: (f64, f64)
}

#[derive(Clone, Copy, Hash, PartialEq, Eq, Deserialize, Serialize)]
pub struct EntryId (u64);

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct EntryDry {
    id: EntryId,
    pub face: Face,
    pub bubble: Bubble,
    pub filter: Filter,
}

pub type Face = String;
pub type Bubble = String;


#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Filter {
    process: ProcessStatus,
    tags: Vec<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize, EnumString, EnumVariantNames, EnumIter, EnumProperty, ToString)]
pub enum ProcessStatus {
    Done,
    Marching,
    Pending,
    Planning,
    New,
}


impl Entry {
    pub fn new() -> Self {
        let stamp = TimeStamp::created();
        let id = EntryId::from_time(&stamp.data);
        Entry {
            dry: EntryDry::new(id),
            timestamps: vec!(stamp)
        }
    }
    pub fn dry(&self) -> EntryDry {
        self.dry.clone()
    }
    pub fn strip(self) -> EntryDry {
        self.dry
    }
    pub fn id(&self) -> EntryId {
        self.dry.id.clone()
    }
    pub fn face(&self) -> &Face {
        &self.dry.face
    }
    pub fn set_face(&mut self, face: Face) {
        self.dry.face = face;
    }
    pub fn bubble(&self) -> &Bubble {
        &self.dry.bubble
    }
    pub fn set_bubble(&mut self, bubble: Bubble) {
        self.dry.bubble = bubble;
    }
    pub fn filter(&self) -> &Filter {
        &self.dry.filter
    }
    pub fn set_filter(&mut self, filter: Filter) {
        self.dry.filter = filter;
    }
}

impl From<EntryDry> for Entry {
    fn from(v: EntryDry) -> Self {
        Entry {
            dry: v,
            timestamps: vec!(TimeStamp::created())
        }
    }
}

impl Default for Entry {
    fn default() -> Self { 
        Entry::new()
    }
    
}

impl EntryDry {
    pub fn new(id: EntryId) -> Self {
        EntryDry {
            id,
            face: Face::new(),
            bubble: Bubble::new(),
            filter: Filter::new()
        }
    }
}

// EntryId impl.

impl std::fmt::Debug for EntryId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> { 
        // write!(f, "[{}.{}]", self.0, self.1) 
        write!(f, "[[{}]]", self.0) 
    }
}

pub trait IdentityHash: Hash + Copy {
    fn from_time(v: &impl TimeRep) -> Self;
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



// Filter impl.


impl Filter {
    pub fn new() -> Self {
        Filter {
            process: ProcessStatus::New,
            tags: Vec::new()
        }
    }
}
