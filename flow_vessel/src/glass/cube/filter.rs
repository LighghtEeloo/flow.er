use serde::{Deserialize, Serialize};

use crate::{Symbol, Tag};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Filter {
    Identity(String),
    Face(String),
    Symbol(Symbol),
    Tag(Tag),
    All,
}

impl Default for Filter {
    fn default() -> Self {
        Filter::All
    }
}
