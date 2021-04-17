use serde::{Serialize, Deserialize};

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct Settings {
    #[serde(default)]
    pub bridge: Bridge,
    pub timezone: i8,
    pub view_mode: ViewMode,
    pub buffer_mode: bool,
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

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ViewMode {
    Desktop,
    Mobile
}

impl Default for ViewMode {
    fn default() -> Self {
        ViewMode::Desktop
    }
}

impl ViewMode {
    pub fn switch(self) -> Self {
        use ViewMode::*;
        match self {
            Desktop => Mobile,
            Mobile => Desktop
        }
    }
    pub fn display(&self) -> &str {
        use ViewMode::*;
        match self {
            Desktop => "Desktop",
            Mobile => "Mobile"
        }
    }
}
