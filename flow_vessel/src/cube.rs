use serde::{Serialize, Deserialize};
use std::time::SystemTime;
use flow_arena::FlowBase;
use crate::{EntityFlow, EntityId, Filter, Router};

pub mod identity;

use identity::{CubeId, CubeIdFactory};

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
            CubeType::TimeView => "time-view",
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
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct Cube {
    pub id: CubeId,
    pub cube_type: CubeType,
    pub obj: Option<EntityId>,
    pub current: Option<EntityId>,
    /// obj & current are first used if requirements are already satisfied; 
    /// if more info is needed, profile is then used.
    pub profile: Option<Profile>,
    pub filters: Vec<Filter>,
}


impl Cube {
    pub fn new(cube_type: CubeType, id_factory: &mut CubeIdFactory) -> Self {
        Self {
            id: id_factory.rotate_id(),
            cube_type,
            ..Self::default()
        }
    }

    pub fn new_router(router: Router, id_factory: &mut CubeIdFactory) -> Self {
        use CubeType::*;
        match router {
            Router::Birdsight => Cube::new(FlowView, id_factory),
            Router::Workspace => Cube::new(ClauseTree, id_factory),
            Router::Promised => Cube::new(PromisedLand, id_factory),
            Router::Calendar => Cube::new(CalendarView, id_factory),
            Router::TimeAnchor => Cube::new(TimeView, id_factory),
            Router::Settings => Cube::new(SettingView, id_factory),
        }
    }
    
    /// if cube.obj is not None and is not in vessel, false; 
    /// otherwise, true.
    pub fn is_valid(&self, flow: &EntityFlow) -> bool {
        self.obj.map_or(true, |obj| 
            flow.contains_node(&obj)
        )
    }
    
    /// fix a cube to a legal state if it's not already.
    pub fn ensure(&mut self, flow: &EntityFlow) -> Option<&mut Self> {
        // current -> Some(<Exist>) | None
        if ! self.current.map_or(false, |cur| 
            flow.contains_node(&cur)
        ) { self.current = None }
        // obj = Some(<Not Exist>) -> clean
        // obj = None | Some(<Exist>) -> keep
        if self.is_valid(flow) {
            Some(self)
        } else {
            None
        }
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
