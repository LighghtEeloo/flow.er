#[allow(dead_code)]
#[allow(unused)]
pub mod cube;
pub mod error;

use serde::{Deserialize, Serialize};
use wasm_timer::{SystemTime, UNIX_EPOCH};
use std::time::Duration;

pub use cube::*;
pub use error::{Result, Error};

pub mod prelude {
    pub use crate::cube::*;
}
