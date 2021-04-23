use serde::{Serialize, Deserialize};

use crate::{Symbol, Tag};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Filter {
    Symbol(Symbol),
    Tag(Tag),
    All
}

impl Default for Filter {
    fn default() -> Self {
        Filter::All
    }
}
