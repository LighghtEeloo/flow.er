use serde::{Deserialize, Serialize};
use std::{
    cmp::Ordering,
    fmt::Debug,
    hash::Hash,
    time::{SystemTime, UNIX_EPOCH},
};

use crate::{now, TimeRep};

pub trait Identity: Default + Debug + Clone + Hash + PartialEq + Eq {
    fn parse_match(&self, attempt: &str) -> bool;
    fn parse_filter(candidates: &Vec<Self>, attempt: &str) -> Option<Self>;
}

const LEN: usize = 5;

#[derive(Clone, Copy, Hash, PartialEq, Eq, Ord, Serialize, Deserialize)]
pub struct TimeUnique {
    time: SystemTime,
    unique: u64,
}

impl PartialOrd for TimeUnique {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let time_cmp = self.time.partial_cmp(&other.time);
        let unique_cmp = self.unique.partial_cmp(&other.unique);
        match (time_cmp, unique_cmp) {
            (Some(Ordering::Equal), Some(Ordering::Equal)) => {
                Some(Ordering::Equal)
            }
            (Some(Ordering::Equal), _) => None,
            (Some(_), _) => time_cmp,
            _ => None,
        }
    }
}

impl Default for TimeUnique {
    fn default() -> Self {
        Self {
            time: UNIX_EPOCH,
            unique: 0,
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
            let hash: String = format!("{:x}", self.unique)
                .as_str()
                .chars()
                .take(LEN)
                .collect();
            write!(f, "(({}))", hash)
        } else {
            // raw
            let hash: String = format!("{:x}", self.unique)
                .as_str()
                .chars()
                .take(LEN)
                .collect();
            write!(f, "[[{}]]", hash)
        }
    }
}

impl Identity for TimeUnique {
    fn parse_match(&self, attempt: &str) -> bool {
        format!("{:x}", self.unique).starts_with(attempt)
    }

    fn parse_filter(candidates: &Vec<Self>, attempt: &str) -> Option<Self> {
        let mut candidates: Vec<Self> = candidates
            .iter()
            .filter_map(|id| {
                if id.parse_match(attempt) {
                    Some(id.clone())
                } else {
                    None
                }
            })
            .collect();
        if candidates.len() == 1 {
            candidates.pop()
        } else if let Some(id) = candidates
            .iter()
            .find(|id| format!("{:x}", id.unique).eq(attempt))
        {
            Some(id.clone())
        } else {
            None
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IdFactory {
    cnt: u64,
}

impl Default for IdFactory {
    fn default() -> Self {
        Self {
            cnt: 0
        }
    }
}

impl IdFactory {
    /// generate id by increment
    pub fn incr_id(&mut self) -> TimeUnique {
        Self::rotate_add(&mut self.cnt);
        TimeUnique {
            time: UNIX_EPOCH,
            unique: self.cnt,
        }
    }
    pub fn time_id(&self) -> TimeUnique {
        TimeUnique {
            time: now(),
            unique: rand::random(),
        }
    }
    pub fn rotate_id(&mut self) -> TimeUnique {
        Self::rotate_add(&mut self.cnt);
        TimeUnique {
            time: now(),
            unique: self.cnt,
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
        assert_eq!(
            id_factory.incr_id(),
            TimeUnique {
                time: UNIX_EPOCH,
                unique: 1
            }
        );
        assert_eq!(
            id_factory.incr_id(),
            TimeUnique {
                time: UNIX_EPOCH,
                unique: 2
            }
        );
        assert_eq!(
            id_factory.incr_id(),
            TimeUnique {
                time: UNIX_EPOCH,
                unique: 3
            }
        );
    }
    #[test]
    fn factory_time() {
        let id_factory = IdFactory::default();
        println!("{:#?}", id_factory.time_id());
        println!("{:#?}", id_factory.time_id());
        println!("{:#?}", id_factory.time_id());
    }
}
