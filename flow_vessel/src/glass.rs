use serde::{Serialize, Deserialize};

use super::EntityId;

/// Describes the app router.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Deserialize, Serialize)]
pub enum Router {
    BirdView,
    Board,
    Todos,
    Calendar,
    TimeAnchor,

    Settings,
}

impl Default for Router {
    fn default() -> Self {
        Router::Board
    }
}

impl Router {
    pub fn type_str(&self) -> &'static str {
        use Router::*;
        match self {
            BirdView => "bird-view",
            Board => "board",
            Todos => "todos",
            Calendar => "calendar",
            TimeAnchor => "time-capsule",
            Settings => "settings"
        }
    }
    pub fn display_str(&self) -> &'static str {
        use Router::*;
        match self {
            BirdView => "BirdView",
            Board => "Board",
            Todos => "Todos",
            Calendar => "Calendar",
            TimeAnchor => "TimeAnchor",
            Settings => "Settings"
        }
    }
}

/// The basic unit of view, containing minimum info for rendering.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Cube {
    /// A single entity's notebook.
    Inkblot {
        obj: EntityId
    },
    /// A single entity and its points.
    PointView {
        obj: EntityId,
        current: usize
    },
    /// A single entity and a todo list view
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



/// A overall layer of routers and cubes.
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct Glass {
    bird_view: Option<Cube>,
    board: Vec<Cube>,
    todos: Option<Cube>,
    calendar: Option<Cube>,
    time_anchor: (),
    settings: (),
}
