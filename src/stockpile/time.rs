use crate::util::*;

pub use wasm_timer::{SystemTime, UNIX_EPOCH};
pub use std::time::Duration;
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

#[derive(Clone, Deserialize, Serialize)]
pub struct TimeStamp {
    pub meta: TimeMeta,
    pub data: TimeTuple
}

#[derive(Clone, Debug, Deserialize, Serialize)]
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

impl Debug for TimeStamp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> std::result::Result<(), fmt::Error> { 
        write!(f, "{:?} - {:?}", self.meta, self.flatten())
    }
}

