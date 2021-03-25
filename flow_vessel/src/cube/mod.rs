use serde::{Serialize, Deserialize};
use std::time::SystemTime;
use crate::{EntityId, now, Router};

mod inkblot {
    use crate::EntityId;
    use super::{Cube, CubeType};

    pub struct Inkblot {
        pub obj: EntityId
    }

    impl From<Inkblot> for Cube {
        fn from(inkblot: Inkblot) -> Self {
            Self {
                cube_type: CubeType::Inkblot,
                obj: Some(inkblot.obj),
                ..Self::default()
            }
        }
    }
}

mod clause_tree;

mod flow_view {
    use crate::EntityId;
    use super::{Cube, CubeType};

    pub struct FlowView {
        pub obj: EntityId,
        pub current: Option<EntityId>
    }

    impl From<FlowView> for Cube {
        fn from(flow: FlowView) -> Self {
            Self {
                cube_type: CubeType::FlowView,
                obj: Some(flow.obj),
                current_id: flow.current,
                ..Self::default()
            }
        }
    }
}

mod calendar_view {
    use std::time::SystemTime;
    use super::{Cube, CubeType};

    pub struct CalendarView {
        pub current: SystemTime
    }

    impl From<CalendarView> for Cube {
        fn from(calendar: CalendarView) -> Self {
            Self {
                cube_type: CubeType::CalendarView,
                time: Some(calendar.current),
                ..Self::default()
            }
        }
    }
}

mod blank {
    use super::{Cube, CubeType};

    pub struct Blank {
        pub alt: String
    }

    impl From<Blank> for Cube {
        fn from(blank: Blank) -> Self {
            Self {
                cube_type: CubeType::Blank,
                alt: Some(blank.alt),
                ..Self::default()
            }
        }
    }
}

pub use inkblot::Inkblot;
pub use clause_tree::ClauseTree;
pub use flow_view::FlowView;
pub use calendar_view::CalendarView;
pub use blank::Blank;


/// The basic unit of view, containing minimum info for rendering.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum CubeType {
    /// A single entity's notebook.
    Inkblot,
    /// A single entity  view
    ClauseTree,
    /// A todo-oriented pool view
    PromisedLand,
    FlowView,
    CalendarView,
    TimeView,
    SettingView,
    Blank
}

impl Default for CubeType {
    fn default() -> Self {
        CubeType::Blank 
    }
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct Cube {
    cube_type: CubeType,
    obj: Option<EntityId>,
    current_id: Option<EntityId>,
    current_idx: Option<usize>,
    time: Option<SystemTime>,
    alt: Option<String>,
}

// template
// impl From<Inkblot> for Cube {
//     fn from(: Inkblot) -> Self {
//         Self {
//             ..Self::default()
//         }
//     }
// }


impl Cube {
    pub fn new(router: Router) -> Self {
        match router {
            Router::BirdView => 
                FlowView {
                    obj: EntityId::default(),
                    current: Some(EntityId::default())
                }.into(),
            Router::Board => 
                ClauseTree { 
                    obj: EntityId::default(),
                    current: None
                }.into(),
            // Router::Promised => PromisedLand,
            Router::Calendar => 
                CalendarView { current: now() }.into(),
            // Router::TimeAnchor => TimeView,
            // Router::Settings => SettingView,
            // Todo..
            _ => Cube::default()
        }
    }
    pub fn is_blank(&self) -> bool {
        if let CubeType::Blank = self.cube_type {
            true
        } else {
            false
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct CubeMeta {
    pub router: Router,
    pub idx: usize,
    pub cube_type: CubeType,
}
