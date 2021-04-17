use serde::{Serialize, Deserialize};
use std::{collections::HashSet, time::SystemTime};
use flow_arena::FlowBase;
use crate::{EntityFlow, EntityId, Router, Vessel, now};

mod clause_tree;

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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Profile {
    Where (Option<EntityId>),
    When (SystemTime),
    Why (String),
    None
}

impl Default for Profile {
    fn default() -> Self {
        Profile::None
    }
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct Cube {
    pub cube_type: CubeType,
    pub obj: Option<EntityId>,
    pub current: Option<EntityId>,
    pub profile: Profile,
}


impl Cube {
    pub fn new(cube_type: CubeType) -> Self {
        Self {
            cube_type,
            ..Self::default()
        }
    }
    pub fn new_router(router: Router) -> Self {
        use CubeType::*;
        match router {
            Router::BirdView => Cube::new(FlowView),
            Router::Board => Cube::new(ClauseTree),
            Router::Promised => Cube::new(PromisedLand),
            Router::Calendar => Cube::new(CalendarView),
            Router::TimeAnchor => Cube::new(TimeView),
            Router::Settings => Cube::new(SettingView),
        }
    }
    pub fn is_empty_blank(&self) -> bool {
        // if let CubeType::Blank = self.cube_type {
        //     let blank: Blank = self.clone().into();
        //     blank.alt.is_empty()
        // } else {
        //     false
        // }
        false
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

pub mod cubes {
    pub use crate::cube::{
    };
}
