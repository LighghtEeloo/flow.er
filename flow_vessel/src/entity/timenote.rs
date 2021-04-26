use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use std::{
    fmt::{self, Debug},
    time::{Duration, SystemTime},
};

use crate::{Tag, TimeRep};

#[derive(Default, Clone, Serialize, Deserialize)]
pub struct TimeNote {
    notes: Vec<(SystemTime, Tag)>,
}

impl TimeNote {
    pub fn new() -> TimeNote {
        TimeNote::default()
    }
    pub fn add_start(&mut self, start: SystemTime) -> &mut Self {
        self.notes.insert(0, (start, Tag::default()));
        self
    }
    pub fn add_end(&mut self, end: SystemTime) -> &mut Self {
        self.notes.push((end, Tag::default()));
        self
    }
    pub fn add_sorted_with_tag(
        &mut self,
        time: SystemTime,
        tag: Tag,
    ) -> &mut Self {
        self.notes.push((time, tag));
        self.notes.sort_by_key(|(t, _)| t.clone());
        self
    }
    pub fn start_t(&self) -> Option<&SystemTime> {
        self.notes.first().map(|note| &note.0)
    }
    pub fn end_t(&self) -> Option<&SystemTime> {
        if self.notes.len() == 1 {
            None
        } else {
            self.notes.last().map(|note| &note.0)
        }
    }
    pub fn length(&self) -> Duration {
        match (self.start_t(), self.end_t()) {
            (Some(s), Some(e)) => {
                e.duration_since(s.clone()).unwrap_or_default()
            }
            _ => Duration::default(),
        }
    }
}

impl TimeRep for TimeNote {
    fn human_local_detail(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[:")?;
        self.start_t().map(|t| t.human_local_detail(f));
        write!(f, " ~ ")?;
        self.end_t().map(|t| t.human_local_detail(f));
        write!(f, ":]")?;
        write!(f, "")
    }

    fn human_local(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[:")?;
        self.start_t().map(|t| t.human_local(f));
        write!(f, " ~ ")?;
        self.end_t().map(|t| t.human_local(f));
        write!(f, ":]")?;
        write!(f, "")
    }
    fn clock_local(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(time) = None.or(self.start_t()).or(self.end_t()) {
            let dt: DateTime<Local> = time.clone().into();
            write!(f, "{}", dt.format("%H:%M:%S").to_string())
        } else {
            write!(f, "len_0")
        }
    }

    fn human_utc(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[:")?;
        self.start_t().map(|t| t.human_utc(f));
        write!(f, " ~ ")?;
        self.end_t().map(|t| t.human_utc(f));
        write!(f, ":]")?;
        write!(f, "")
    }
}

impl Debug for TimeNote {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
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
        time.add_start(SystemTime::now());
        sleep(Duration::from_millis(u_dur));
        time.add_end(SystemTime::now());
        let dur = time.length();
        println!("{:?}", dur);
        assert!(dur > Duration::from_millis(u_dur));
        println!("{:?}", time)
    }
}
