use crate::util::*;

pub use wasm_timer::{SystemTime, UNIX_EPOCH};
pub use std::time::Duration;
// Timestamp Area

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

    // pub fn snapshot(entry: &Entry) -> Self {
    //     TimeStamp {
    //         meta: TimeMeta::Snapshot(entry.dry()),
    //         data: TimeStamp::stamping()
    //     }
    // }
}

impl<T> Debug for TimeStamp<T> 
where T: Clone + Debug
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> { 
        write!(f, "{:?} - {:?}", self.time, self.universal())
    }
}


// TimeRep trait

pub trait TimeRep {
    fn new() -> Self;
    /// To SystemTime.
    fn universal(&self) -> SystemTime;
    /// To u128.
    fn flatten(&self) -> u128 {
        let time = self.universal()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_nanos();
        time
    }
}

impl TimeRep for TimeTuple {
    fn new() -> Self {
        // Using nano_secs as timestamp.
        let time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards");
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
