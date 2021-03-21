use serde::{Serialize, Deserialize};

use super::EntityId;

/// Describes the app router.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Deserialize, Serialize)]
pub enum Router {
    BirdView,
    Board,
    Promised,
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
            Promised => "promised",
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
            Promised => "Promised",
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
        current: Option<usize>
    },
    /// A single entity and a todo list view
    TodoList {
        obj: EntityId,
        current: Option<usize>
    },
    FlowView {
        obj: EntityId,
        current: EntityId
    },
    CalendarView {
        current: EntityId
    },
    TimeView,
    SettingView,
    Blank {
        alt: String
    }
}

impl Default for Cube {
    fn default() -> Self {
        Cube::Blank {
            alt: format!("Clean and innocent.")
        }
    }
}


/// A overall layer of routers and cubes.
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct Glass {
    bird_view: Cube,
    board: Vec<Cube>,
    promised: Cube,
    calendar: Cube,
    time_anchor: Cube,
    settings: Cube,
}

impl Glass {
    pub fn router(&self, router: Router) -> Vec<Cube> {
        if router == Router::Board {
            self.board.clone()
        } else {
            use Cube::*;
            let cube = match router {
                Router::BirdView => self.bird_view.clone(),
                Router::Promised => self.promised.clone(),
                Router::Calendar => self.calendar.clone(),
                Router::TimeAnchor => self.time_anchor.clone(),
                Router::Settings => self.settings.clone(),
                _ => Cube::default()
            };
            [cube].to_vec()
        }
    }
}
