use serde::{Serialize, Deserialize};

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct Settings {
    #[serde(default)]
    pub bridge: Bridge,
    pub timezone: i8,
    pub view_mode: ViewMode,
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ViewMode {
    Desktop,
    Mobile
}

impl Default for ViewMode {
    fn default() -> Self {
        ViewMode::Desktop
    }
}
