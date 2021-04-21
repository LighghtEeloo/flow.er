use serde::{Serialize, Deserialize};
use std::{collections::{HashMap, HashSet}};


use crate::{Cube, CubeId, CubeIdFactory, CubeMeta, EntityFlow, settings::WorkSpaceMode};


/// Describes the app router.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Deserialize, Serialize)]
pub enum Router {
    Birdsight,
    Workspace,
    Promised,
    Calendar,
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
            Calendar => "calendar",
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
            Calendar => "Calendar",
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
            Calendar => "static/icons/calendar.svg",
            TimeAnchor => "static/icons/history.svg",
            Settings => "static/icons/settings.svg",
        }
    }
    /// returns all possible routers in a Vec<Router>
    pub fn vec_router(workspace_mode: WorkSpaceMode) -> Vec<Self> {
        use WorkSpaceMode::*;
        use Router::*;
        match workspace_mode {
            Pure => vec! [ Workspace ],
            Full => vec! [
                // BirdView,
                Workspace,
                Promised,
                // Calendar,
                // TimeAnchor,
                
                Settings,
            ],
            Manual(vec) => vec
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
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Glass {
    #[serde(default)]
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
        let router_map = Router::vec_router(WorkSpaceMode::default()).iter()
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
    fn locate_cube_id(&self, meta: CubeMeta) -> Option<CubeId> {
        self.router_map.get(&meta.router)
        .map(|vec| vec.get(meta.idx))
        .flatten().cloned()
    }
    pub fn locate_cube(&self, meta: CubeMeta) -> Option<Cube> {
        let id = self.locate_cube_id(meta);
        id.map(|id| 
            self.cube_map.get(&id)
        ).flatten().cloned()
    }
    pub fn get_cube(&self, id: CubeId) -> Option<Cube> {
        self.cube_map.get(&id).cloned()
    }
    pub fn show_router_cubes(&self) -> Vec<(CubeMeta, CubeId, Cube)> {
        let vec = 
            self.router_map
            .get(&self.router).cloned()
            .unwrap_or_default();
        vec.into_iter()
        .filter_map(|id| 
            self.cube_map.get(&id).cloned().map(|c| (id, c))
        ).enumerate().map(|(idx, (id, cube))| 
            (CubeMeta { router: self.router, idx }, id , cube)
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
    pub fn place_cube(&mut self, cube: CubeId, meta: CubeMeta) -> Result<(),()> {
        let router = meta.router;
        let vec = self.ensured_router(router);
        vec.retain(|id| id != &cube);
        let idx = if meta.idx > vec.len() {
            return Err(())
        } else { meta.idx };
        vec.insert(idx, cube);
        Ok(())
    }

    // pub fn push_cube(&mut self, cube: CubeId, router: Router) -> Result<(),()> {
    //     let idx = self.ensured_router(router).len();
    //     let meta = CubeMeta { 
    //         router,
    //         idx,
    //     };
    //     self.place_cube(cube, meta)
    // }

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

    /// ensure and clean all invalid cubes.
    pub fn refresh(&mut self, flow: &EntityFlow, workspace_mode: WorkSpaceMode) {
        self.ensure_router(workspace_mode);
        self.clean(flow);
        self.ensure_fallback()
    }

    fn ensure_router(&mut self, workspace_mode: WorkSpaceMode) {
        let router_vec = Router::vec_router(workspace_mode);
        router_vec.into_iter().for_each(|router| {
            self.ensured_router(router);
        });
    }

    /// cleans up all invalid cubes in both router_map and cube_map; assumes done ensure_router().
    fn clean(&mut self, flow: &EntityFlow) {
        // first check all cubes are valid
        self.cube_map.retain(|_, cube| {
            cube.ensure(flow);
            cube.is_valid_obj(flow)
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
    use super::*;
    use Router::*;
    fn make_glass() {

    }
    #[test]
    fn glass_insert_delete() {

    }
    #[test]
    fn glass_refresh() {

    }
    fn glass_fallback() {

    }
}
