use serde::{Serialize, Deserialize};
use std::time::SystemTime;
use flow_arena::FlowBase;
use crate::{EntityFlow, EntityId, Filter, Router};

pub mod identity;

use identity::{CubeId, CubeIdFactory};

/// The basic unit of view, containing minimum info for rendering. Note that all use cases are strict.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum CubeType {
    /// A single entity's notebook. Uses obj & ?current.
    Inkblot,
    /// A single entity view. Uses obj & ?current.
    ClauseTree,
    /// A todo-oriented pool view. Uses ?obj and ?current.
    PromisedLand,
    /// A flow view from a point. Uses obj & ?current.
    FlowView,
    /// A calendar with agenda. Uses profile.
    CalendarView,
    /// A version control panel.
    TimeView,
    /// A setting panel.
    SettingView,
    /// Blank Page with simple notes. Uses ?obj & profile.
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


/// new Cube
impl Cube {
    pub fn id(&self) -> &CubeId {
        &self.id
    }

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
}

/// validating
impl Cube {
    /// if cube.obj is Some but not in vessel, false; so does cube.current
    ///
    /// if cube_type doesn't allow, false
    ///
    /// otherwise, true.
    pub fn is_valid_obj(&self, flow: &EntityFlow) -> bool {
        use CubeType::*;
        let legal = match (self.cube_type, self.obj, self.current, self.profile.clone()) {
            (Inkblot,Some(_),_,None) |
            (ClauseTree,Some(_),_,None) |
            (PromisedLand,_,_,None) |
            (FlowView,Some(_),_,None) |
            (CalendarView,None,None,Some(Profile::Where(_))) |
            (CalendarView,None,None,Some(Profile::When(_))) |
            (TimeView,None,None,None) |
            (SettingView,None,None,None) |
            (Blank,_,None,Some(Profile::Why(_))) => true,
            _ => false
        };
        let legal = self.current.map_or(legal, |current| 
            flow.contains_node(&current)
        );
        // if obj is Some, perform contain check
        let legal = self.obj.map_or(legal, |obj| 
            flow.contains_node(&obj)
        );
        legal
    }
    
    /// fix a cube to a legal state if it's not already.
    pub fn ensure(&mut self, flow: &EntityFlow) -> Option<&mut Self> {
        // current -> Some(<Exist>) | None
        if ! self.current.map_or(false, |cur| 
            flow.contains_node(&cur)
        ) { self.clear_current(); }
        // obj = Some(<Not Exist>) -> clean
        // obj = None | Some(<Exist>) -> keep
        if self.is_valid_obj(flow) {
            Some(self)
        } else {
            None
        }
    }
}


/// obj, current & profile
impl Cube {
    pub fn with_obj(mut self, obj: EntityId) -> Self {
        self.obj = Some(obj);
        self
    }
    pub fn set_obj(&mut self, obj: EntityId) -> &mut Self {
        self.obj = Some(obj);
        self
    }
    pub fn clear_obj(&mut self) -> &mut Self {
        self.obj = None;
        self
    }
    pub fn with_current(mut self, current: EntityId) -> Self {
        self.current = Some(current);
        self
    }
    pub fn set_current(&mut self, current: EntityId) -> &mut Self {
        self.current = Some(current);
        self
    }
    pub fn clear_current(&mut self) -> &mut Self {
        self.current = None;
        self
    }
    pub fn with_profile(mut self, profile: Profile) -> Self {
        self.profile = Some(profile);
        self
    }
    pub fn set_profile(&mut self, profile: Profile) -> &mut Self {
        self.profile = Some(profile);
        self
    }
    pub fn clear_profile(&mut self) -> &mut Self {
        self.profile = None;
        self
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

#[cfg(test)]
mod tests {
    use crate::{Entity, EntityIdFactory};
    use super::*;

    #[test]
    fn cube_create() {
        let mut entity_factory = EntityIdFactory::default();
        let entitys: Vec<Entity> = (0..20).map(|_| Entity::new_rotate(&mut entity_factory)).collect();
        println!("{:?}", entitys.iter().map(|x| x.id()).collect::<Vec<&EntityId>>());
        let mut cube_factory = CubeIdFactory::default();
        let mut cube = Cube::new_router(Router::Workspace, &mut cube_factory).with_obj(entitys[0].id().clone());
        println!("{:#?}", cube.obj);
        cube.set_obj(entitys[2].id().clone());
        println!("{:#?}", cube.obj);
    }
}
