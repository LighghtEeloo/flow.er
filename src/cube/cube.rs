use crate::cube::prelude::*;

use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::time::{SystemTime, UNIX_EPOCH};

pub struct Cube {

}

// Entry Area

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Entry {
    internal: EntryDry,
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
            internal: EntryDry::new(id),
            timestamps: vec!(stamp)
        }
    }
    pub fn strip(&self) -> EntryDry {
        self.internal.clone()
    }
}

impl From<EntryDry> for Entry {
    fn from(v: EntryDry) -> Self {
        Entry {
            internal: v,
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
        let time = v.flatten()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_nanos();
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

