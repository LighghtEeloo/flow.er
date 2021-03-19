use serde::{Serialize, Deserialize};

use super::EntityId;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Deserialize, Serialize)]
pub enum Router {
    Cube,
    Flow,
    Calendar,
    TimeCapsule,

    Settings,
}

impl Default for Router {
    fn default() -> Self {
        Router::Cube
    }
}

/// Cube is the basic unit of view, 
/// containing the minimum info for rendering.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Cube {
    /// A single entity
    Inkblot {
        obj: EntityId
    },
    TodoList {
        obj: EntityId,
        current: usize
    },
    FlowView {
        obj: EntityId,
        current: EntityId
    },
    Calendar {
        current: EntityId
    },
}


