use serde::{Serialize, Deserialize};
use serde_json::to_vec;
use std::time::SystemTime;

use super::{EntityId, now};

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
        current: SystemTime
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

impl Cube {
    pub fn new(router: Router) -> Self {
        use Cube::*;
        match router {
            Router::BirdView => 
                FlowView {
                    obj: EntityId::default(),
                    current: EntityId::default()
                },
            Router::Board => Cube::default(),
            Router::Promised => Cube::default(),
            Router::Calendar => CalendarView { current: now() },
            Router::TimeAnchor => TimeView,
            Router::Settings => SettingView,
        }
    }
    pub fn is_blank(&self) -> bool {
        if let Cube::Blank{ alt: _ } = self {
            true
        } else {
            false
        }
    }
}


/// A overall layer of routers and cubes.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Glass {
    bird_view: Cube,
    board: Vec<Cube>,
    promised: Cube,
    calendar: Cube,
    time_anchor: Cube,
    settings: Cube,
}

impl Default for Glass {
    fn default() -> Self {
        use Cube::*;
        Glass {
            bird_view: 
                FlowView {
                    obj: EntityId::default(),
                    current: EntityId::default()
                },
            // board: vec![Blank { alt: format!("Focus on a node to start.") }],
            // Test..
            board: vec![TodoList { current: None, obj: EntityId::default() }],
            promised: Cube::default(),
            calendar: CalendarView { current: now() },
            time_anchor: TimeView,
            settings: SettingView,
        }
    }
}

impl Glass {
    pub fn router(&self, router: Router) -> Vec<Cube> {
        use Cube::Blank;
        if router == Router::Board {
            if self.board.is_empty() {
                vec![Blank { alt: format!("Focus on a node to start.") }]
            } else {
                self.board.clone()
            }
        } else {
            let cube = match router {
                Router::BirdView => self.bird_view.clone(),
                Router::Promised => self.promised.clone(),
                Router::Calendar => self.calendar.clone(),
                Router::TimeAnchor => self.time_anchor.clone(),
                Router::Settings => self.settings.clone(),
                _ => Blank { alt: format!("Error.") }
            };
            [cube].to_vec()
        }
    }
}
