use crate::cube::prelude::*;

use std::collections::HashMap;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use yew::web_sys;


#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Cube {
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
            entries: HashMap::new(),
            relation: RelationModel::Linear(Vec::new())
        }
    }
    pub fn get() {
        todo!()
    }
}






#[derive(Debug)]
pub enum Identifier {
    Id(EntryId),
    Index(usize)
}

// Entry Area

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Entry {
    dry: EntryDry,
    timestamps: Vec<TimeStamp>
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

pub type EntryId = String;

pub trait TimeRep {
    fn flatten(&self) -> f64 {
        unimplemented!()
    }
    fn hashable(time: f64) -> (u64, u32) {
        (time.trunc() as u64, (time.fract() * 1e8) as u32)
    }
    fn stamping() -> (u64, u32) {
        // Using nano_secs as timestamp.
        let time: f64 = web_sys::window()
            .expect("should have a Window")
            .performance()
            .expect("should have a Performance")
            .now();
        // Debug..
        web_sys::console::log_1(&time.to_string().into());
        TimeRep::hashable(time)
    }
}
impl TimeRep for (u64, u32) {
    fn flatten(&self) -> f64 {
        let sec = self.0;
        let sub_nano = self.1;
        sec as f64 + sub_nano as f64 / 1e-8
    }
}
impl TimeRep for TimeStamp {
    fn flatten(&self) -> f64 {
        let sec = self.data.0;
        let sub_nano = self.data.1;
        sec as f64 + sub_nano as f64 / 1e-8
    }
}

pub trait IdentityHash {
    fn from_time(v: &impl TimeRep) -> Self;
}
impl IdentityHash for EntryId {
    fn from_time(v: &impl TimeRep) -> Self {
        let time: f64 = v.flatten();
        let time: (u64, u32) = TimeRep::hashable(time);
        let mut s = DefaultHasher::new();
        time.hash(&mut s);
        format!("{:x}", s.finish())
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

