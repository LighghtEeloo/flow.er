use std::{fmt::Debug, hash::Hash};
use std::time::{SystemTime, UNIX_EPOCH};
use serde::{Serialize, Deserialize};

use crate::{TimeRep, now};

pub trait Identity: Default + Debug + Clone + Hash + PartialEq + Eq {}

const LEN: usize = 5;


#[derive(Clone, Copy, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub struct TimeUnique {
    time: SystemTime,
    unique: u64
}

impl Default for TimeUnique {
    fn default() -> Self {
        Self {
            time: UNIX_EPOCH,
            unique: 0
        }
    }
}

impl Debug for TimeUnique {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if f.alternate() {
            // pretty
            write!(f, "[[")?;
            if self.time == UNIX_EPOCH {
                write!(f, "0_0")?;
            } else {
                self.time.human_local(f)?;
            }
            write!(f, "]]")?;
            let hash: String = format!("{:x}", self.unique).as_str().chars().take(LEN).collect();
            write!(f, "(({}))", hash)
        } else {
            // raw
            let hash: String = format!("{:x}", self.unique).as_str().chars().take(LEN).collect();
            write!(f, "[[{}]]", hash)
        }
    }
}

impl Identity for TimeUnique {}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct IdFactory {
    cnt: u64
}

impl IdFactory {
    /// generate id by increment
    pub fn incr_id(&mut self) -> TimeUnique {
        Self::rotate_add(&mut self.cnt);
        TimeUnique {
            time: UNIX_EPOCH,
            unique: self.cnt
        }
    }
    pub fn time_id(&self) -> TimeUnique {
        TimeUnique {
            time: now(),
            unique: rand::random()
        }
    }
    pub fn rotate_id(&mut self) -> TimeUnique {
        Self::rotate_add(&mut self.cnt);
        TimeUnique {
            time: now(),
            unique: self.cnt
        }
    }
    fn rotate_add(cnt: &mut u64) {
        *cnt = cnt.checked_add(1).map_or(1, |x| x)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default() {
        let de = TimeUnique::default();
        println!("{:?}", de);
        println!("{:#?}", de);
    }
    #[test]
    fn factory_incr() {
        let mut id_factory = IdFactory::default();
        assert_eq!(id_factory.incr_id(), TimeUnique { time: UNIX_EPOCH, unique: 1 });
        assert_eq!(id_factory.incr_id(), TimeUnique { time: UNIX_EPOCH, unique: 2 });
        assert_eq!(id_factory.incr_id(), TimeUnique { time: UNIX_EPOCH, unique: 3 });
    }
    #[test]
    fn factory_time() {
        let id_factory = IdFactory::default();
        println!("{:#?}", id_factory.time_id());
        println!("{:#?}", id_factory.time_id());
        println!("{:#?}", id_factory.time_id());
    }
}
