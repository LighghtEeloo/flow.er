use crate::util::*;

use wasm_timer;
use std::time;
use std::time::Duration;
use chrono::prelude::*;

pub type TimeTuple = (u64, u32);

#[derive(Clone, Deserialize, Serialize)]
pub struct TimeStamp<T> 
where T: Clone + Debug
{
    pub time: TimeTuple,
    pub data: TimeMeta<T>,
}


#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum TimeMeta<T> 
where T: Clone + Debug
{
    Created,
    Snapshot(T),
    // Todo: Snapshot design.
    Diffshot(String),
}

impl<T> TimeStamp<T> 
where T: Clone + Debug
{
    pub fn new_snapshot(v: &T) -> Self {
        Self {
            time: TimeTuple::new(),
            data: TimeMeta::Snapshot(v.clone()),
        }
    }
}

impl<T> Debug for TimeStamp<T> 
where T: Clone + Debug
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> { 
        write!(f, "{:?} ...", self.time)
    }
}

impl<T> fmt::Display for TimeStamp<T> 
where T: Clone + Debug
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> { 
        let duration = self.universal().duration_since(wasm_timer::UNIX_EPOCH).expect("time went backwards");
        let dt: DateTime<Local> = (time::UNIX_EPOCH + duration).into();
        // Todo: Use arg controlled timezone / js -> html element.
        write!(f, "{}", dt.with_timezone(&FixedOffset::east(8*3600)).format("%Y-%m-%d %a %H:%M:%S").to_string())
    }
}

impl<T> PartialEq for TimeStamp<T>
where T: Clone + Debug
{
    fn eq(&self, other: &Self) -> bool {
        self.time_eq(other)
    }
}

impl<T> Eq for TimeStamp<T> where T: Clone + Debug {}


// TimeRep trait

pub trait TimeRep {
    fn new() -> Self;
    /// To SystemTime.
    fn universal(&self) -> wasm_timer::SystemTime;
    /// To u128.
    fn flatten(&self) -> u128 {
        to_duration(self.universal()).as_nanos()
    }
    fn flatten_tuple(&self) -> TimeTuple {
        let time = to_duration(self.universal());
        (time.as_secs(), time.subsec_nanos())
    }
    fn time_eq(&self, other: &impl TimeRep) -> bool {
        self.flatten() == other.flatten()
    }
}

fn to_duration(sys_time: wasm_timer::SystemTime) -> Duration {
    sys_time.duration_since(wasm_timer::UNIX_EPOCH).expect("Time went backwards")
}

impl TimeRep for TimeTuple {
    fn new() -> Self {
        // Using nano_secs as timestamp.
        let time = to_duration(wasm_timer::SystemTime::now());
        (time.as_secs(), time.subsec_nanos())
    }
    fn universal(&self) -> wasm_timer::SystemTime {
        let sec = self.0;
        let sub_nano = self.1;
        wasm_timer::UNIX_EPOCH + Duration::from_nanos(sec*1e9 as u64 + sub_nano as u64)
    }
}

impl<T> TimeRep for TimeStamp<T> 
where T: Clone + Debug
{
    fn new() -> Self {
        TimeStamp {
            time: TimeTuple::new(),
            data: TimeMeta::<T>::Created,
        }
    }
    fn universal(&self) -> wasm_timer::SystemTime {
        self.time.universal()
    }
}
