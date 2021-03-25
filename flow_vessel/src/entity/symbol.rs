use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Symbol {
    ProcessTracker(Process),
    Linted(Lint),
    Innocent
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Process {
    Done,
    Marching,
    Pending,
    Planning,
    New,
}

use Process::*;
impl Process {
    pub fn type_str(&self) -> String {
        match self {
            Done => "Done",
            Marching => "Marching",
            Pending => "Pending",
            Planning => "Planning",
            New => "New",
        }.to_string()
    }
    pub fn vec_all() -> Vec<Self> {
        vec! {
            New,
            Planning,
            Pending,
            Marching,
            Done,
        }
    }
    pub fn type_src(&self) -> String {
        format!("static/icons/Process/{}.svg", Self::type_str(self))
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Lint {
    Numberic,
    Upper,
    Lower,
    Greek,

    Circle,
    Square,
    Dash
}

// Todo: impl Lint.
impl Lint {
    pub fn display(&self, idx: usize) -> String {
        "▣".into()
    }
}
