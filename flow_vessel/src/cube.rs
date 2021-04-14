use serde::{Serialize, Deserialize};
use std::{collections::HashSet, time::SystemTime};
use flow_arena::FlowMap;
use crate::{EntityFlow, EntityId, Router, Vessel, now};

mod inkblot;
mod clause_tree;
mod promise_land {
    use super::{Cube, CubeType, CubeMember};
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
    impl CubeMember for PromisedLand {}
}
mod flow_view;
mod calendar_view;
mod time_view {
    use super::{Cube, CubeType, CubeMember};
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
    impl CubeMember for TimeView {}
}
mod setting_view {
    use super::{Cube, CubeType, CubeMember};
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
    impl CubeMember for SettingView {}
}
mod blank;

pub use inkblot::Inkblot;
pub use clause_tree::{ClauseTreeCube, ClauseTreeCore};
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

impl CubeType {
    pub fn type_str(&self) -> &'static str {
        match self {
            CubeType::Inkblot => "inkblot",
            CubeType::ClauseTree => "clause-tree",
            CubeType::PromisedLand => "promised-land",
            CubeType::FlowView => "flow-view",
            CubeType::CalendarView => "calendar-view",
            CubeType::TimeView => "",
            CubeType::SettingView => "setting-view",
            CubeType::Blank => "blank",
        }
    }
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct Cube {
    pub cube_type: CubeType,
    pub obj: Option<EntityId>,
    pub current: Option<EntityId>,
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
                ClauseTreeCube { 
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
    pub fn is_empty_blank(&self) -> bool {
        if let CubeType::Blank = self.cube_type {
            let blank: Blank = self.clone().into();
            blank.alt.is_empty()
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

/// generated on site; isn't contained in a Vec<Cube>
#[derive(Debug, Clone, Copy)]
pub struct CubeMeta {
    pub router: Router,
    pub idx: usize,
}

impl CubeMeta {
    pub fn incr_new(&self) -> Self {
        Self {
            idx: self.idx + 1,
            ..self.clone()
        }
    }
}

pub trait CubeMember {
    fn member_traverse(&self, _vessel: &Vessel) -> HashSet<EntityId> {
        HashSet::new()
    }
}

impl CubeMember for Cube {
    fn member_traverse(&self, vessel: &Vessel) -> HashSet<EntityId> {
        let cube = self.clone();
        match self.cube_type {
            CubeType::Inkblot => {
                let cu: Inkblot = cube.into();
                cu.member_traverse(vessel)
            }
            CubeType::ClauseTree => {
                let cu: ClauseTreeCube = cube.into();
                cu.member_traverse(vessel)
            }
            CubeType::PromisedLand => {
                let cu: PromisedLand = cube.into();
                cu.member_traverse(vessel)
            }
            CubeType::FlowView => {
                let cu: FlowView = cube.into();
                cu.member_traverse(vessel)
            }
            CubeType::CalendarView => {
                let cu: CalendarView = cube.into();
                cu.member_traverse(vessel)
            }
            CubeType::TimeView => {
                let cu: TimeView = cube.into();
                cu.member_traverse(vessel)
            }
            CubeType::SettingView => {
                let cu: SettingView = cube.into();
                cu.member_traverse(vessel)
            }
            CubeType::Blank => {
                let cu: Blank = cube.into();
                cu.member_traverse(vessel)
            }
        }
    }
}
