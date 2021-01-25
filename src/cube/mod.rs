#[allow(dead_code)]
#[allow(unused)]
pub mod error;
pub mod loader;
pub mod item;
pub mod collection;
pub mod cube;

pub use item::{TaskItem, Identifier, TaskItemId};
pub use collection::{Collection};
pub use error::{Result, Error};
pub use cube::{Cube, Filter};

pub type TracerPath = std::path::PathBuf;

pub mod prelude {
    pub use crate::cube::*;
}
