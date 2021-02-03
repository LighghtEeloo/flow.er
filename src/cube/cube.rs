use crate::cube::prelude::*;

use std::collections::HashMap;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};


#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Cube {
    pub name: String,
    pub entries: HashMap<EntryId, Entry>,
    pub relation: RelationModel
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum RelationModel {
    Linear(Vec<EntryId>),
    Graph
    // Todo: Garph RelationModel.
}

impl Cube {
    pub fn new() -> Self {
        Cube {
            name: String::new(),
            entries: HashMap::new(),
            relation: RelationModel::Linear(Vec::new())
        }
    }
    pub fn empty(&self) -> bool {
        self.entries.len() == 0
    }
    pub fn get() {
        todo!()
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
    face: Face,
    bubble: Bubble,
    filter: Filter,
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
    pub fn strip(&self) -> EntryDry {
        self.dry.clone()
    }
    pub fn dry(&mut self) -> &mut EntryDry {
        &mut self.dry
    }
    pub fn id(&self) -> EntryId {
        self.dry.id.clone()
    }
    pub fn face(&mut self) -> &mut Face {
        &mut self.dry.face
    }
    pub fn bubble(&mut self) -> &mut Bubble {
        &mut self.dry.bubble
    }
    pub fn filter(&mut self) -> &mut Filter {
        &mut self.dry.filter
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

pub type EntryId = (u64, u32);

pub trait TimeRep {
    fn flatten(&self) -> SystemTime {
        unimplemented!()
    }
    fn stamping() -> (u64, u32) {
        // Using nano_secs as timestamp.
        let time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards");
        (time.as_secs(), time.subsec_nanos())
    }
}
impl TimeRep for (u64, u32) {
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

pub trait IdentityHash {
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
        (time.as_secs(), time.subsec_nanos())
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

#[derive(Clone, Debug, Deserialize, Serialize)]
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

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct TimeStamp {
    meta: TimeMeta,
    data: (u64, u32)
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum TimeMeta {
    Created,
    Snapshot(EntryDry)
    // Todo: Snapshot design.
}

impl TimeStamp {
    pub fn created() -> Self {
        TimeStamp {
            meta: TimeMeta::Created,
            data: TimeStamp::stamping()
        }
    }

    pub fn snapshot(entry: &Entry) -> Self {
        TimeStamp {
            meta: TimeMeta::Snapshot(entry.strip()),
            data: TimeStamp::stamping()
        }
    }
}

