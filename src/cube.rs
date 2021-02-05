#[allow(dead_code)]
#[allow(unused)]

use crate::prelude::*;
pub use std::collections::HashMap;
pub use wasm_timer::{SystemTime, UNIX_EPOCH};
pub use std::time::Duration;

// pub use error::{Result, Error};

pub mod prelude {
    pub use crate::cube::*;
}

use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};


#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Cube {
    pub name: String,
    pub locked: bool,
    pub entries: HashMap<EntryId, Entry>,
    pub relation: RelationModel,
}

#[derive(Clone, Debug, Deserialize, Serialize, EnumString, EnumVariantNames, EnumIter, EnumProperty, ToString)]
pub enum RelationModel {
    Linear(Vec<EntryId>),
    Graph
    // Todo: Garph RelationModel.
}

impl Cube {
    pub fn new() -> Self {
        Cube {
            name: String::new(),
            locked: false,
            entries: HashMap::new(),
            relation: RelationModel::Linear(Vec::new())
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
                vec.insert(0, id)
            }
            _ => ()
        }
    }
    fn chain(&mut self, new_comer: EntryId, host: EntryId) {
        use RelationModel::*;
        match &mut self.relation {
            Linear(vec) => {
                let pos = vec.into_iter().position(|x| x.clone() == host);
                if let Some(p) = pos {
                    vec.insert(p+1, new_comer)
                } else {
                    vec.push(new_comer)
                }
            }
            _ => ()
        }
    }
}





// Entry Area

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Entry {
    dry: EntryDry,
    timestamps: Vec<TimeStamp>,
    // Todo: Add positional info.
    // position: (f64, f64)
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct EntryDry {
    id: EntryId,
    pub face: Face,
    pub bubble: Bubble,
    pub filter: Filter,
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

#[derive(Clone, Copy, Hash, PartialEq, Eq, Deserialize, Serialize)]
pub struct EntryId (u64, u32);

pub trait IdentityHash: Hash + Copy {
    fn from_time(v: &impl TimeRep) -> Self;
}
impl IdentityHash for EntryId {
    fn from_time(v: &impl TimeRep) -> Self {
        // let time = v.flatten()
        //     .duration_since(UNIX_EPOCH)
        //     .expect("Time went backwards")
        //     .as_nanos();
        // let mut s = DefaultHasher::new();
        // time.hash(&mut s);
        // format!("{:x}", s.finish())
        let time = v.flatten()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards");
        EntryId (time.as_secs(), time.subsec_nanos())
    }
}

impl std::fmt::Debug for EntryId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> { 
        write!(f, "[{}.{}]", self.0, self.1) 
    }
}


pub type Face = String;
pub type Bubble = String;

// Filter Area

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

impl Filter {
    pub fn new() -> Self {
        Filter {
            process: ProcessStatus::New,
            tags: Vec::new()
        }
    }
}

// Timestamp Area

pub type TimeTuple = (u64, u32);
// pub type TimeTuple = EntryId;

pub trait TimeRep {
    fn flatten(&self) -> SystemTime {
        unimplemented!()
    }
    fn stamping() -> TimeTuple {
        // Using nano_secs as timestamp.
        let time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards");
        (time.as_secs(), time.subsec_nanos())
    }
}
impl TimeRep for TimeTuple {
    fn flatten(&self) -> SystemTime {
        let sec = self.0;
        let sub_nano = self.1;
        UNIX_EPOCH + Duration::from_nanos(sec*1e9 as u64 + sub_nano as u64)
    }
}
impl TimeRep for TimeStamp {
    fn flatten(&self) -> SystemTime {
        let sec = self.data.0;
        let sub_nano = self.data.1;
        UNIX_EPOCH + Duration::from_nanos(sec*1e9 as u64 + sub_nano as u64)
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct TimeStamp {
    meta: TimeMeta,
    data: TimeTuple
}

#[derive(Clone, Debug, Deserialize, Serialize, EnumString, EnumVariantNames, EnumIter, EnumProperty, ToString)]
pub enum TimeMeta {
    Created,
    // Snapshot(EntryDry)
    // Todo: Snapshot design.
}

impl TimeStamp {
    pub fn created() -> Self {
        TimeStamp {
            meta: TimeMeta::Created,
            data: TimeStamp::stamping()
        }
    }

    // pub fn snapshot(entry: &Entry) -> Self {
    //     TimeStamp {
    //         meta: TimeMeta::Snapshot(entry.dry()),
    //         data: TimeStamp::stamping()
    //     }
    // }
}

