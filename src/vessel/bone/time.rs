/// `time.rs` contains:
/// 1. trait TimeRep.
/// 2. TimeStamp: a tuple struct storing the secs from epoch as u64 and nano_secs as u32.
/// 3. TimeCapsule: a struct with .time (TimeStamp) and .data (TimeMeta)
/// 4. TimeMeta: a enum of 
///     A. Created,
///     B. Snapshot(T),
///     C. Diffshot(String, TimeStamp),


use crate::util::*;
use crate::time_util::*;


// TimeRep trait

pub trait TimeRep {
    fn now() -> Self;
    /// To SystemTime.
    fn universal(&self) -> SystemTime;
    /// To u128.
    fn flatten(&self) -> u128 {
        to_duration(self.universal()).as_nanos()
    }
    fn flatten_tuple(&self) -> TimeStamp {
        let time = to_duration(self.universal());
        TimeStamp (time.as_secs(), time.subsec_nanos())
    }
    fn time_eq(&self, other: &impl TimeRep) -> bool {
        self.flatten() == other.flatten()
    }
}

// TimeStamp

#[derive(Clone, Hash, PartialEq, Eq, Deserialize, Serialize)]
pub struct TimeStamp (u64, u32);
impl fmt::Debug for TimeStamp 
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> { 
        let dt: DateTime<Local> = self.universal().into();
        write!(f, "{}", dt.to_string())
    }
}
impl fmt::Display for TimeStamp 
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> { 
        let dt: DateTime<Local> = self.universal().into();
        write!(f, "{} - UTC", dt.format("%Y-%m-%d %a %H:%M:%S").to_string())
    }
}

fn to_duration(sys_time: SystemTime) -> Duration {
    sys_time.duration_since(UNIX_EPOCH).expect("Time went backwards")
}
fn wasm_to_duration(sys_time: wasm_timer::SystemTime) -> Duration {
    sys_time.duration_since(wasm_timer::UNIX_EPOCH).expect("Time went backwards")
}

impl TimeRep for TimeStamp {
    fn now() -> Self {
        // Using nano_secs as timestamp.
        let time = wasm_to_duration(wasm_timer::SystemTime::now());
        TimeStamp (time.as_secs(), time.subsec_nanos())
    }
    fn universal(&self) -> SystemTime {
        let sec = self.0;
        let sub_nano = self.1;
        UNIX_EPOCH + Duration::from_nanos(sec*1e9 as u64 + sub_nano as u64)
    }
}


// TimeCapsule

#[derive(Clone, Deserialize, Serialize)]
pub struct TimeCapsule<T> 
where T: Clone + Debug
{
    pub time: TimeStamp,
    pub data: TimeMeta<T>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum TimeMeta<T> 
where T: Clone + Debug
{
    Created,
    Snapshot(T),
    // Todo: Snapshot design.
    Diffshot(String, TimeStamp),
}


impl<T> fmt::Display for TimeCapsule<T> 
where T: Clone + Debug
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> { 
         let dt: DateTime<Local> = self.universal().into();
        // Todo: Use arg controlled timezone / js -> html element.
        write!(f, "Captured at {}", dt.with_timezone(&FixedOffset::east(8*3600)).format("%Y-%m-%d %a %H:%M:%S").to_string())
    }
}
impl<T> PartialEq for TimeCapsule<T>
where T: Clone + Debug
{
    fn eq(&self, other: &Self) -> bool {
        self.time_eq(other)
    }
}
impl<T> Eq for TimeCapsule<T> where T: Clone + Debug {}


impl<T> TimeRep for TimeCapsule<T> 
where T: Clone + Debug
{
    fn now() -> Self {
        TimeCapsule {
            time: TimeStamp::now(),
            data: TimeMeta::<T>::Created,
        }
    }
    fn universal(&self) -> SystemTime {
        self.time.universal()
    }
}

impl<T> TimeCapsule<T> 
where T: Clone + Debug
{
    pub fn new_snapshot(v: &T) -> Self {
        Self {
            time: TimeStamp::now(),
            data: TimeMeta::Snapshot(v.clone()),
        }
    }
}

