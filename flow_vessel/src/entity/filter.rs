use serde::{Serialize, Deserialize};
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Filter {
    All
}

impl Default for Filter {
    fn default() -> Self {
        Filter::All
    }
}
