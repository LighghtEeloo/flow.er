#[allow(dead_code)]
#[allow(unused)]
pub mod cube;
pub mod loader;
pub mod error;

use serde::{Deserialize, Serialize};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

pub use cube::*;
pub use loader::Barge;
pub use error::{Result, Error};

pub type TracerPath = std::path::PathBuf;

pub mod prelude {
    pub use crate::cube::*;
}
