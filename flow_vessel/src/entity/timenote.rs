use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use std::{
    fmt::Debug,
    time::{Duration, SystemTime},
};

use crate::TimeRep;

#[derive(Default, Clone, Serialize, Deserialize)]
pub struct TimeNote {
    notes: Vec<SystemTime>,
}

impl TimeNote {
    pub fn new() -> TimeNote {
        TimeNote::default()
    }
    pub fn set_start(&mut self, start: SystemTime) -> &mut Self {
        self.notes.insert(0, start);
        self
    }
    pub fn set_end(&mut self, end: SystemTime) -> &mut Self {
        self.notes.push(end);
        self
    }
    pub fn length(&self) -> Duration {
        match (self.notes.first(), self.notes.last()) {
            (Some(s), Some(e)) => {
                e.duration_since(s.clone()).unwrap_or_default()
            }
            _ => Duration::default(),
        }
    }
}

impl TimeRep for TimeNote {
    fn human_local_detail(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        write!(f, "[:")?;
        self.notes.first().map(|t| t.human_local_detail(f));
        write!(f, " ~ ")?;
        self.notes.last().map(|t| t.human_local_detail(f));
        write!(f, ":]")?;
        write!(f, "")
    }

    fn human_local(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[:")?;
        self.notes.first().map(|t| t.human_local(f));
        write!(f, " ~ ")?;
        self.notes.last().map(|t| t.human_local(f));
        write!(f, ":]")?;
        write!(f, "")
    }
    fn clock_local(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(time) = None.or(self.notes.first()).or(self.notes.last()) {
            let dt: DateTime<Local> = time.clone().into();
            write!(f, "{}", dt.format("%H:%M:%S").to_string())
        } else {
            write!(f, "len_0")
        }
    }

    fn human_utc(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[:")?;
        self.notes.first().map(|t| t.human_utc(f));
        write!(f, " ~ ")?;
        self.notes.last().map(|t| t.human_utc(f));
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
