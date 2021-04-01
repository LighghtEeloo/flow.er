use serde::{Serialize, Deserialize};

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct Settings {
    #[serde(default)]
    pub bridge: Bridge,
    pub timezone: i8,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Bridge {
    Local,
    Linked {
        addr: String,
        port: u16
    }
}

impl Default for Bridge {
    fn default() -> Self {
        Bridge::Local
    }
}
