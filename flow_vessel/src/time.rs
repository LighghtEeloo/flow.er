use std::time::SystemTime;
use chrono::prelude::*;
pub trait TimeRep {
    fn human_local_detail(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result;
    fn human_local(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result;
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
    fn human_utc(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let dt: DateTime<Utc> = self.clone().into();
        write!(f, "{} (UTC)", dt.format("%Y-%m-%d %H:%M:%S").to_string())
    }
}
