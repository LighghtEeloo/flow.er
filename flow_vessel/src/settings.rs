use serde::{Serialize, Deserialize};

use crate::Router;

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct Settings {
    #[serde(default)]
    pub bridge: Bridge,
    #[serde(default)]
    pub timezone: i8,
    #[serde(default)]
    pub view_mode: ViewMode,
    #[serde(default)]
    pub workspace_mode: WorkspaceMode,
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


#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum WorkspaceMode {
    Full,
    Pure,
    Manual (Vec<Router>)
}

impl Default for WorkspaceMode {
    fn default() -> Self {
        WorkspaceMode::Full
    }
}

impl WorkspaceMode {
    pub fn switch(self) -> Self {
        use WorkspaceMode::*;
        match self {
            Full => Pure,
            Pure => Full,
            _ => Full
        }
    }
    pub fn display(&self) -> String {
        use WorkspaceMode::*;
        match self {
            Full => format!("Full"),
            Pure => format!("Pure"),
            Manual (vec) => format!("Maunal: {:?}", vec)
        }
    }
}
