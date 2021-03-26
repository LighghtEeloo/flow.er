use serde::{Serialize, Deserialize};
use std::time::SystemTime;
use flow_arena::Flow;
use crate::{EntityFlow, EntityId, Router, now};

mod inkblot;
mod clause_tree;
mod promise_land {
    use super::{Cube, CubeType};
    pub struct PromisedLand;
    impl Into<Cube> for PromisedLand {
        fn into(self) -> Cube {
            Cube {
                cube_type: CubeType::PromisedLand,
                ..Cube::default()
            }
        }
    }
    impl From<Cube> for PromisedLand {
        fn from(_cube: Cube) -> Self {
            Self
        }
    }
}
mod flow_view;
mod calendar_view;
mod time_view {
    use super::{Cube, CubeType};
    pub struct TimeView;
    impl Into<Cube> for TimeView {
        fn into(self) -> Cube {
            Cube {
                cube_type: CubeType::TimeView,
                ..Cube::default()
            }
        }
    }
    impl From<Cube> for TimeView {
        fn from(_cube: Cube) -> Self {
            Self
        }
    }
}
mod setting_view {
    use super::{Cube, CubeType};
    pub struct SettingView;
    impl Into<Cube> for SettingView {
        fn into(self) -> Cube {
            Cube {
                cube_type: CubeType::SettingView,
                ..Cube::default()
            }
        }
    }
    impl From<Cube> for SettingView {
        fn from(_cube: Cube) -> Self {
            Self
        }
    }
}
mod blank;

pub use inkblot::Inkblot;
pub use clause_tree::ClauseTree;
pub use flow_view::FlowView;
pub use promise_land::PromisedLand;
pub use calendar_view::CalendarView;
pub use time_view::TimeView;
pub use setting_view::SettingView;
pub use blank::Blank;


/// The basic unit of view, containing minimum info for rendering.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum CubeType {
    /// A single entity's notebook.
    Inkblot,
    /// A single entity  view.
    ClauseTree,
    /// A todo-oriented pool view.
    PromisedLand,
    /// A flow view from a point.
    FlowView,
    /// A calendar with agenda.
    CalendarView,
    /// A version control panel.
    TimeView,
    /// A setting panel.
    SettingView,
    /// Blank Page with simple notes.
    Blank
}

impl Default for CubeType {
    fn default() -> Self {
        CubeType::Blank 
    }
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct Cube {
    pub cube_type: CubeType,
    pub obj: Option<EntityId>,
    pub current_id: Option<EntityId>,
    pub current_idx: Option<usize>,
    pub time: Option<SystemTime>,
    pub alt: Option<String>,
}


impl Cube {
    pub fn new(router: Router) -> Self {
        match router {
            Router::BirdView => 
                FlowView {
                    obj: EntityId::default(),
                    current: Some(EntityId::default())
                }.into(),
            Router::Board => 
                // Debug..
                ClauseTree { 
                    obj: EntityId::default(),
                    current: None
                }.into(),
            Router::Promised => PromisedLand.into(),
            Router::Calendar => 
                CalendarView { current: now() }.into(),
            Router::TimeAnchor => TimeView.into(),
            Router::Settings => SettingView.into(),
        }
    }
    pub fn is_blank(&self) -> bool {
        if let CubeType::Blank = self.cube_type {
            true
        } else {
            false
        }
    }
    /// if cube.obj is not None and is not in vessel, false; 
    /// otherwise, true.
    pub fn is_valid(&self, flow: &EntityFlow) -> bool {
        if let Some(obj) = self.obj {
            flow.node(&obj).is_some()
        } else { true }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct CubeMeta {
    pub router: Router,
    pub idx: usize,
    pub cube_type: CubeType,
}
