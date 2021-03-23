use std::{fmt::Debug, time::{SystemTime, Duration}};
#[cfg(target_arch = "wasm32")]
use std::time::UNIX_EPOCH;
use chrono::{DateTime, Local, Utc};
use serde::{Serialize, Deserialize};

pub trait TimeRep {
    fn human_local_detail(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result;
    fn human_local(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result;
    fn clock_local(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result;
    fn human_utc(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result;
}

impl TimeRep for SystemTime {
    fn human_local_detail(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let dt: DateTime<Local> = self.clone().into();
        write!(f, "{} (Local)", dt.format("%Y-%m-%d %a %H:%M:%S").to_string())
    }
    fn human_local(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let dt: DateTime<Local> = self.clone().into();
        write!(f, "{} (Local)", dt.format("%Y-%m-%d %H:%M:%S").to_string())
    }
    fn clock_local(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let dt: DateTime<Local> = self.clone().into();
        write!(f, "{}", dt.format("%H:%M:%S").to_string())
    }
    fn human_utc(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let dt: DateTime<Utc> = self.clone().into();
        write!(f, "{} (UTC)", dt.format("%Y-%m-%d %H:%M:%S").to_string())
    }
}

#[cfg(target_arch = "wasm32")]
pub fn now() -> SystemTime {
    let dur = wasm_timer::SystemTime::now()
        .duration_since(wasm_timer::UNIX_EPOCH)
        .expect("time went backwards");
    UNIX_EPOCH + dur
}

#[cfg(not(target_arch = "wasm32"))]
pub fn now() -> SystemTime {
    SystemTime::now()
}

#[derive(Default)]
#[derive(Clone, Serialize, Deserialize)]
pub struct TimeNote {
    start: Option<SystemTime>,
    end: Option<SystemTime>
}

impl TimeNote {
    pub fn new() -> TimeNote {
        TimeNote::default()
    }
    pub fn set_start(&mut self, start: SystemTime) -> &mut Self {
        self.start = Some(start);
        self
    }
    pub fn set_end(&mut self, end: SystemTime) -> &mut Self {
        self.end = Some(end);
        self
    }
    pub fn length(&self) -> Duration {
        match (self.start, self.end) {
            (Some(s), Some(e)) => e.duration_since(s).unwrap_or(Duration::new(0, 0)),
            _ => Duration::new(0, 0)
        }
    }
}

impl TimeRep for TimeNote {
    fn human_local_detail(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[:")?;
        self.start.map(|t: SystemTime| t.human_local_detail(f));
        write!(f, " ~ ")?;
        self.end.map(|t: SystemTime| t.human_local_detail(f));
        write!(f, ":]")?;
        write!(f, "")
    }

    fn human_local(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[:")?;
        self.start.map(|t: SystemTime| t.human_local(f));
        write!(f, " ~ ")?;
        self.end.map(|t: SystemTime| t.human_local(f));
        write!(f, ":]")?;
        write!(f, "")
    }
    fn clock_local(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(time) = None.or(self.start).or(self.end) {
            let dt: DateTime<Local> = time.into();
            write!(f, "{}", dt.format("%H:%M:%S").to_string())
        } else {
            write!(f, "len_0")
        }
    }

    fn human_utc(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[:")?;
        self.start.map(|t: SystemTime| t.human_utc(f));
        write!(f, " ~ ")?;
        self.end.map(|t: SystemTime| t.human_utc(f));
        write!(f, ":]")?;
        write!(f, "")
    }
}

impl Debug for TimeNote {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if f.alternate() {
            self.human_local_detail(f)
        } else {
            self.human_local(f)
        }
    }
}


pub mod display {
    use std::fmt::Display;
    use super::TimeRep;    

    pub struct TimeClockLocal<T> (T);

    impl<T: Sized> From<T> for TimeClockLocal<T> {
        fn from(t: T) -> Self {
            Self (t)
        }
    }

    impl<T: Sized + TimeRep> Display for TimeClockLocal<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            self.0.clock_local(f)
        }
    }

    pub struct TimeHumanLocal<T> (T);

    impl<T: Sized> From<T> for TimeHumanLocal<T> {
        fn from(t: T) -> Self {
            Self (t)
        }
    }

    impl<T: Sized + TimeRep> Display for TimeHumanLocal<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            self.0.human_local(f)
        }
    }

    pub struct TimeHumanUTC<T> (T);

    impl<T: Sized> From<T> for TimeHumanUTC<T> {
        fn from(t: T) -> Self {
            Self (t)
        }
    }

    impl<T: Sized + TimeRep> Display for TimeHumanUTC<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            self.0.human_utc(f)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread::sleep;
    #[test]
    fn time_log() {
        let u_dur = 200;
        let mut time = TimeNote::new();
        time.set_start(SystemTime::now());
        sleep(Duration::from_millis(u_dur));
        time.set_end(SystemTime::now());
        let dur = time.length();
        println!("{:?}", dur);
        assert!(dur > Duration::from_millis(u_dur));
        println!("{:?}", time)
    }
}
