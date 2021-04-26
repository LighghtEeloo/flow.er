use crate::{EntityFlow, EntityId, Router};
use flow_arena::FlowBase;
use serde::{Deserialize, Serialize};
use std::time::SystemTime;

pub mod filter;
pub mod identity;

use filter::Filter;

/// The basic unit of view, containing minimum info for rendering. Note that all use cases are strict.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum CubeType {
    /// A single entity's notebook. Uses obj & ?current.
    Inkblot,
    /// A single entity view. Uses obj.
    NodeView,
    /// A recursive entity view. Uses obj & ?current.
    ClauseTree,
    /// A todo-oriented pool view. Uses ?obj and ?current.
    PromisedLand,
    /// A flow view from a point. Uses obj & ?current.
    FlowView,
    /// A agenda with agenda. Uses profile.
    AgendaView,
    /// A version control panel.
    TimeView,
    /// A setting panel.
    SettingView,
    /// Blank Page with simple notes. Uses ?obj & profile.
    Blank,
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
            CubeType::NodeView => "node-view",
            CubeType::ClauseTree => "clause-tree",
            CubeType::PromisedLand => "promised-land",
            CubeType::FlowView => "flow-view",
            CubeType::AgendaView => "agenda-view",
            CubeType::TimeView => "time-view",
            CubeType::SettingView => "setting-view",
            CubeType::Blank => "blank",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Profile {
    Where(Option<EntityId>),
    When(SystemTime),
    Why(String),
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct Cube {
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
    pub fn new(cube_type: CubeType) -> Self {
        Self {
            cube_type,
            ..Self::default()
        }
    }

    pub fn new_router(router: Router) -> Self {
        use CubeType::*;
        match router {
            Router::Birdsight => Cube::new(FlowView),
            Router::Workspace => Cube::new(ClauseTree),
            Router::Promised => Cube::new(PromisedLand),
            Router::Agenda => Cube::new(AgendaView),
            Router::TimeAnchor => Cube::new(TimeView),
            Router::Settings => Cube::new(SettingView),
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
    pub fn is_valid_cube(&self, flow: &EntityFlow) -> bool {
        // if obj is Some, perform contain check
        let is_obj = self.obj.map_or(true, |obj| flow.contains_node(&obj));
        // if current is Some, perform contain check
        let is_current = self
            .current
            .map_or(true, |current| flow.contains_node(&current));
        use CubeType::*;
        let legal = match (
            self.cube_type,
            self.obj,
            self.current,
            self.profile.clone(),
        ) {
            (Inkblot, Some(_), _, None)
            | (NodeView, _, None, None)
            | (ClauseTree, Some(_), _, None)
            | (PromisedLand, _, _, None)
            | (FlowView, Some(_), _, None)
            | (AgendaView, None, None, Some(Profile::Where(_)))
            | (AgendaView, None, None, Some(Profile::When(_)))
            | (TimeView, None, None, None)
            | (SettingView, None, None, None)
            | (Blank, _, None, Some(Profile::Why(_))) => true,
            _ => false,
        };
        is_obj && is_current && legal
    }

    /// fix a cube to a legal state if it's not already.
    pub fn ensure(&mut self, flow: &EntityFlow) -> Option<&mut Self> {
        // current -> Some(<Exist>) | None
        if !self.current.map_or(false, |cur| flow.contains_node(&cur)) {
            self.clear_current();
        }
        // obj = Some(<Not Exist>) -> clean
        // obj = None | Some(<Exist>) -> keep
        if self.is_valid_cube(flow) {
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
#[derive(Default, Debug, Clone, Copy)]
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Entity, EntityIdFactory};

    #[test]
    fn cube_create() {
        let mut entity_factory = EntityIdFactory::default();
        let entitys: Vec<Entity> = (0..20)
            .map(|_| Entity::new_rotate(&mut entity_factory))
            .collect();
        println!(
            "{:?}",
            entitys.iter().map(|x| x.id()).collect::<Vec<&EntityId>>()
        );
        let mut cube = Cube::new_router(Router::Workspace)
            .with_obj(entitys[0].id().clone());
        println!("{:#?}", cube.obj);
        cube.set_obj(entitys[2].id().clone());
        println!("{:#?}", cube.obj);
    }
}
