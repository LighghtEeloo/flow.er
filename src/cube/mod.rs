#[allow(dead_code)]
#[allow(unused)]
pub mod cube;
pub mod error;

pub use serde::{Deserialize, Serialize};
pub use wasm_timer::{SystemTime, UNIX_EPOCH};
pub use std::time::Duration;

pub use cube::*;
pub use error::{Result, Error};

pub mod prelude {
    pub use crate::cube::*;
}
