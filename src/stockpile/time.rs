use crate::util::*;

pub use wasm_timer::{SystemTime, UNIX_EPOCH};
pub use std::time::Duration;

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
        write!(f, "{:?} - {:?}", self.time, self.universal())
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
    fn universal(&self) -> SystemTime;
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

fn to_duration(sys_time: SystemTime) -> Duration {
    sys_time.duration_since(UNIX_EPOCH).expect("Time went backwards")
}

impl TimeRep for TimeTuple {
    fn new() -> Self {
        // Using nano_secs as timestamp.
        let time = to_duration(SystemTime::now());
        (time.as_secs(), time.subsec_nanos())
    }
    fn universal(&self) -> SystemTime {
        let sec = self.0;
        let sub_nano = self.1;
        UNIX_EPOCH + Duration::from_nanos(sec*1e9 as u64 + sub_nano as u64)
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
    fn universal(&self) -> SystemTime {
        self.time.universal()
    }
}
