use serde::{Serialize, Deserialize};
use std::{collections::{HashMap, HashSet}};


use crate::{Cube, CubeId, CubeIdFactory, CubeType, EntityFlow, settings::WorkSpaceMode};


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
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Glass {
    #[serde(default)]
    pub router: Router,
    factory: CubeIdFactory,
    router_map: HashMap<Router, Vec<CubeId>>,
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

impl Glass {
    /// return a new cube with id
    ///
    /// ```
    /// # use flow_vessel::*;
    /// let mut glass = Glass::default();
    ///
    /// // use with_... to create:
    /// let cube = glass.new_cube(CubeType::FlowView)
    ///     .with_obj(EntityId::default());
    ///
    /// // verifying:
    /// let cube_made = Cube {
    ///     id: cube.id().clone(),
    ///     obj: Some(EntityId::default()),
    ///     ..Cube::default()
    /// };
    ///
    /// assert_eq!(cube.id, cube_made.id);
    /// assert_eq!(cube.obj, cube_made.obj);
    /// 
    /// ```
    pub fn new_cube(&mut self, cube_type: CubeType) -> Cube {
        Cube {
            ..Cube::new(cube_type, &mut self.factory)   
        }
    }
//     pub fn get_cube_vec(&self) -> Vec<Cube> {
//         let vec = 
//             self.map.get(&self.router).cloned()
//             .unwrap_or(Vec::new());
//         if vec.is_empty() {
//             vec![Cube::default()]
//         } else {
//             vec
//         }
//     }
//     pub fn get_cube(&mut self, meta: CubeMeta) -> Option<Cube> {
//         let router = meta.router;
//         let vec = self.ensured(router);
//         let idx = meta.idx;
//         vec.get(idx).cloned()
//     }
//     /// inserts a cube within a safe idx
//     pub fn insert_cube(&mut self, cube: Cube, meta: CubeMeta) -> Result<(),()> {
//         let router = meta.router;
//         let vec = self.ensured(router);
//         let idx = if meta.idx > vec.len() {
//             return Err(())
//         } else { meta.idx };
//         self.map.get_mut(&router).map(|vec|{
//             vec.insert(idx, cube);
//         });
//         Ok(())
//     }
//     pub fn push_cube(&mut self, cube: Cube, router: Router) -> Result<(),()> {
//         let idx = self.ensured(router).len();
//         let meta = CubeMeta { 
//             router,
//             idx,
//         };
//         self.insert_cube(cube, meta)
//     }
//     /// removes a cube within a safe idx
//     pub fn remove_cube(&mut self, meta: CubeMeta) -> Result<Cube,()> {
//         let router = meta.router;
//         let vec = self.ensured(router);
//         let idx = if meta.idx >= vec.len() {
//             return Err(())
//         } else { meta.idx };
//         let cube = self.map.get_mut(&router).map(|vec|{
//             vec.remove(idx)
//         }).expect("glass.ensured() failed.");
//         Ok(cube)
//     }
//     pub fn swap_cube(&mut self, meta_1: CubeMeta, meta_2: CubeMeta) -> Result<(),()> {
//         let cube = if let Some(cube) = self.get_cube(meta_2) {
//             cube
//         } else { return Err(()) };
//         let cube = self.replace_cube(cube, meta_1)?;
//         self.replace_cube(cube, meta_2)?;
//         Ok(())
//     }
//     /// replaces the cube at meta with a new one
//     pub fn replace_cube(&mut self, cube: Cube, meta: CubeMeta) -> Result<Cube,()> {
//         let cube_ = self.remove_cube(meta)?;
//         self.insert_cube(cube, meta)?;
//         Ok(cube_)
//     }

    /// get cube_ids of a router ensured
    fn ensured_router(&mut self, router: Router) -> &mut Vec<CubeId> {
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
                let cube = Cube::new_router(router, &mut factory);
                vec.push(cube.id().clone());
                cubes.push((cube.id().clone(), cube));
            }
        }
        self.factory = factory;
        self.cube_map.extend(cubes);
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use Router::*;
//     #[test]
//     fn glass_update() {
//         let mut glass = Glass::default();
//         switch(Workspace, &mut glass);
//         let mut vessel = Vessel::default();
//         vessel.glass_refresh();
//     }
//     fn switch(router: Router, glass: &mut Glass) {
//         glass.router = router;
//         println!("{:#?}", glass);
//         println!("{:?}: {:?}", router, glass.get_cube_vec());
//     }
// }
