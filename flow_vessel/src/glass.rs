use serde::{Serialize, Deserialize};
use std::{collections::{HashMap, HashSet}};
use crate::{EntityFlow, settings::WorkspaceMode};

pub mod cube;
pub mod silhouette;
mod ser_de;

use cube::{
    identity::{CubeId, CubeIdFactory},
    {Cube, CubeMeta}
};

/// Describes the app router.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Deserialize, Serialize)]
pub enum Router {
    Birdsight,
    Workspace,
    Promised,
    Agenda,
    TimeAnchor,

    Settings,
}

impl Default for Router {
    fn default() -> Self {
        Router::Workspace
    }
}

impl Router {
    pub fn type_str(&self) -> &'static str {
        use Router::*;
        match self {
            Birdsight => "birdsight",
            Workspace => "workspace",
            Promised => "promised",
            Agenda => "agenda",
            TimeAnchor => "time-capsule",
            Settings => "settings",
        }
    }
    pub fn display_str(&self) -> &'static str {
        use Router::*;
        match self {
            Birdsight => "Birdsight",
            Workspace => "Workspace",
            Promised => "Promised",
            Agenda => "Agenda",
            TimeAnchor => "TimeAnchor",
            Settings => "Settings",
        }
    }
    pub fn src_str(&self) -> &'static str {
        use Router::*;
        match self {
            Birdsight => "static/icons/branch.svg",
            Workspace => "static/icons/hexagons.svg",
            Promised => "static/icons/calendar.svg",
            Agenda => "static/icons/calendar.svg",
            TimeAnchor => "static/icons/history.svg",
            Settings => "static/icons/settings.svg",
        }
    }
    /// returns all possible routers in a Vec<Router>
    pub fn vec_router(workspace_mode: &WorkspaceMode) -> Vec<Self> {
        use WorkspaceMode::*;
        use Router::*;
        match workspace_mode {
            Pure => vec! [ Workspace ],
            Full => vec! [
                // BirdView,
                Workspace,
                Promised,
                // Agenda,
                // TimeAnchor,
                
                Settings,
            ],
            Manual(vec) => {
                if vec.is_empty() {
                    vec! [ Router::default() ]
                } else {
                    vec.clone()
                }
            }
        }
    }
}

/// A overall layer of routers and cubes.
/// 
/// Heavily relies refresh() to keep the router_map and cube_map in correct state.
/// 
/// (e.g. delete by simply removing from either map)
/// 
/// So please call glass.refresh() after flow / glass alteration!
#[derive(Debug, Clone)]
pub struct Glass {
    pub router: Router,
    factory: CubeIdFactory,
    /// each space in `router_map` (a `Vec<CubeId>`) holds no same id within this vec.
    router_map: HashMap<Router, Vec<CubeId>>,
    /// `CubeId` can always find one and only one `Cube` in `cube_map`.
    cube_map: HashMap<CubeId, Cube>,
}

impl Default for Glass {
    fn default() -> Self {
        let factory = CubeIdFactory::default();
        let router_map = Router::vec_router(&WorkspaceMode::default()).iter()
            .map(|&router | {
                (router, Vec::new())
            }).collect();
        let cube_map = HashMap::default();
        Self {
            router: Router::default(),
            factory,
            router_map,
            cube_map
        }
    }
}

/// cube-based operations
impl Glass {
    pub fn get_cube(&self, id: CubeId) -> Option<Cube> {
        self.cube_map.get(&id).cloned()
    }
    pub fn locate_cube_meta(&self, id: CubeId) -> Vec<CubeMeta> {
        self.router_map.iter().filter_map(|(&router, vec)| {
            vec.iter().position(|x| x == &id)
            .map(|idx| CubeMeta {
                router,
                idx
            })
        }).collect()
    }
    fn visit_for_cube_id(&self, meta: CubeMeta) -> Option<CubeId> {
        self.router_map.get(&meta.router)
        .map(|vec| vec.get(meta.idx))
        .flatten().cloned()
    }
    pub fn visit_for_cube(&self, meta: CubeMeta) -> Option<Cube> {
        let id = self.visit_for_cube_id(meta);
        id.map(|id| 
            self.cube_map.get(&id)
        ).flatten().cloned()
    }
    pub fn show_router_cubes(&self) -> Vec<(CubeMeta, CubeId, Cube)> {
        self.show_cubes(self.router)
    }
    pub fn show_cubes(&self, router: Router) -> Vec<(CubeMeta, CubeId, Cube)> {
        let vec = 
            self.router_map
            .get(&router).cloned()
            .unwrap_or_default();
        vec.into_iter()
        .filter_map(|id| 
            self.cube_map.get(&id).cloned().map(|c| (id, c))
        ).enumerate().map(|(idx, (id, cube))| 
            (CubeMeta { router: router, idx }, id , cube)
        ).collect()
    }
    pub fn add_cube(&mut self, cube: Cube) -> CubeId {
        let id = self.factory.rotate_id();
        self.cube_map.insert(id, cube);
        id
    }
    pub fn del_cube(&mut self, id: CubeId) -> Result<Cube, ()> {
        self.cube_map.remove(&id).ok_or(())
    }
}

/// cube_id-based operations
impl Glass {
    /// places a cube within a safe idx
    pub fn place_cube(&mut self, cube_id: CubeId, meta: CubeMeta) -> Result<(),()> {
        let router = meta.router;
        let vec = self.ensured_router(router);
        vec.retain(|id| id != &cube_id);
        let idx = if meta.idx > vec.len() {
            return Err(())
        } else { meta.idx };
        vec.insert(idx, cube_id);
        Ok(())
    }

    pub fn push_cube(&mut self, cube_id: CubeId, router: Router) -> Result<(),()> {
        let idx = self.ensured_router(router).len();
        let meta = CubeMeta { 
            router,
            idx,
        };
        self.place_cube(cube_id, meta)
    }

    /// removes a cube within a safe idx
    pub fn remove_cube(&mut self, meta: CubeMeta) -> Result<CubeId,()> {
        let router = meta.router;
        let vec = self.ensured_router(router);
        let idx = if meta.idx >= vec.len() {
            return Err(())
        } else { meta.idx };
        Ok(vec.remove(idx))
    }
}

/// ensuring operations
impl Glass {
    /// get cube_ids of a router ensured
    pub fn ensured_router(&mut self, router: Router) -> &mut Vec<CubeId> {
        self.router_map.entry(router).or_default()
    }

    /// ensure and clean all invalid cubes; ensure valid router.
    pub fn refresh(&mut self, flow: &EntityFlow, workspace_mode: &WorkspaceMode) {
        self.ensure_router(workspace_mode);
        // Todo: decide whether it stays.
        // self.purge_router(workspace_mode);
        self.clean(flow);
        self.ensure_fallback()
    }

    fn ensure_router(&mut self, workspace_mode: &WorkspaceMode) {
        let router_vec = Router::vec_router(workspace_mode);
        router_vec.iter().for_each(|&router| {
            self.ensured_router(router);
        });
        if ! (router_vec.contains(&self.router)) {
            self.router = router_vec.get(0).cloned()
                .expect("At least one router should be shown.");
        }
    }

    /// removes not_desired router; not generally desired, kept here just in case
    pub fn purge_router(&mut self, workspace_mode: &WorkspaceMode) {
        let router_vec = Router::vec_router(workspace_mode);
        self.router_map.retain(|id, _| router_vec.contains(id));
    }

    /// cleans up all invalid cubes in both router_map and cube_map; assumes done ensure_router().
    fn clean(&mut self, flow: &EntityFlow) {
        // first check all cubes are valid
        self.cube_map.retain(|_, cube| {
            cube.ensure(flow);
            cube.is_valid_cube(flow)
        });

        // then remove all cube_ids in router_map which doesn't exist
        let cube_set: HashSet<CubeId> = self.cube_map.keys().cloned().collect();
        let mut router_set = HashSet::new();
        for vec in self.router_map.values_mut() {
            vec.retain(|id| cube_set.contains(id));
            router_set.extend(vec.iter().cloned());
        }

        // finally compare and remove the un-referenced cubes in cube_map
        for id in cube_set.difference(&router_set) {
            self.cube_map.remove(id);
        }
    }

    /// ensures fallback options if is_empty; assumes done clean().
    fn ensure_fallback(&mut self) {
        let mut factory = self.factory.clone();
        let mut cubes = Vec::new();
        for (&router, vec) in self.router_map.iter_mut() {
            if vec.is_empty() {
                let cube = Cube::new_router(router);
                let id = factory.rotate_id();
                vec.push(id);
                cubes.push((id, cube));
            }
        }
        self.factory = factory;
        self.cube_map.extend(cubes);
    }
}

#[cfg(test)]
mod tests {
    use flow_arena::{FlowMap, FlowNode};

    use crate::{CubeType, Entity, EntityIdFactory, Profile};

    use super::*;
    fn make_glass() -> (EntityFlow, WorkspaceMode, Glass) {
        let mut glass = Glass::default();
        let workspace_mode = WorkspaceMode::Pure;
        glass.purge_router(&workspace_mode);
        
        let mut flow = EntityFlow::default();
        let mut factory = EntityIdFactory::default();

        // legal FlowView
        {
            let entity = Entity::new_rotate(&mut factory);
            let obj = entity.id().clone();
            flow.grow(FlowNode::from_id(obj, entity)).expect("grow failed");
            let flow_view = Cube::new(CubeType::FlowView).with_obj(obj);
            let flow_view = glass.add_cube(flow_view);
            let meta = CubeMeta { router: Router::Workspace, idx: 0 };
            glass.place_cube(flow_view, meta).expect("place_cube failed");
        }

        // legal PromisedLand
        {
            let promised_land = Cube::new(CubeType::PromisedLand);
            let promised_land = glass.add_cube(promised_land);
            let meta = CubeMeta { router: Router::Workspace, idx: 0 };
            glass.place_cube(promised_land, meta).expect("place_cube failed");
        }

        // legal PromisedLand, illegal placement
        {
            let promised_land = Cube::new(CubeType::PromisedLand);
            let promised_land = glass.add_cube(promised_land);
            let meta = CubeMeta { router: Router::Workspace, idx: 3 };
            glass.place_cube(promised_land, meta).expect_err("place_cube shouldn't succeed");
        }

        // illegal PromisedLand
        {
            let entity = Entity::new_rotate(&mut factory);
            let obj = entity.id().clone();
            flow.grow(FlowNode::from_id(obj, entity)).expect("grow failed");
            let promised_land = Cube::new(CubeType::PromisedLand)
                .with_obj(obj)
                .with_profile(Profile::Why("Delibrately using cube illegally.".into()));
            println!("is_valid_cube: {}", promised_land.is_valid_cube(&flow));
            let promised_land = glass.add_cube(promised_land);
            let meta = CubeMeta { router: Router::Workspace, idx: 2 };
            glass.place_cube(promised_land, meta).expect("place_cube failed");
        }

        glass.refresh(&flow, &workspace_mode);
        (flow, workspace_mode, glass)
    }
    #[test]
    fn glass_all() {
        println!("{:#?}", make_glass());
    }
}
