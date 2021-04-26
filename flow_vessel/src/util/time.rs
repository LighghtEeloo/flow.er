use chrono::{DateTime, Local, Utc};
use std::time::SystemTime;
#[cfg(target_arch = "wasm32")]
use std::time::UNIX_EPOCH;

pub trait TimeRep {
    fn human_local_detail(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result;
    fn human_local(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result;
    fn clock_local(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result;
    fn human_utc(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result;
}

impl TimeRep for SystemTime {
    fn human_local_detail(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        let dt: DateTime<Local> = self.clone().into();
        write!(
            f,
            "{} (Local)",
            dt.format("%Y-%m-%d %a %H:%M:%S").to_string()
        )
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

pub mod display {
    use super::TimeRep;
    use std::fmt::Display;

    pub struct TimeClockLocal<T>(T);

    impl<T: Sized> From<T> for TimeClockLocal<T> {
        fn from(t: T) -> Self {
            Self(t)
        }
    }

    impl<T: Sized + TimeRep> Display for TimeClockLocal<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            self.0.clock_local(f)
        }
    }

    pub struct TimeHumanLocal<T>(T);

    impl<T: Sized> From<T> for TimeHumanLocal<T> {
        fn from(t: T) -> Self {
            Self(t)
        }
    }

    impl<T: Sized + TimeRep> Display for TimeHumanLocal<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            self.0.human_local(f)
        }
    }

    pub struct TimeHumanUTC<T>(T);

    impl<T: Sized> From<T> for TimeHumanUTC<T> {
        fn from(t: T) -> Self {
            Self(t)
        }
    }

    impl<T: Sized + TimeRep> Display for TimeHumanUTC<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            self.0.human_utc(f)
        }
    }
}
