use serde::{Serialize, Deserialize};
use std::{collections::HashMap};


use crate::CubeIdFactory;

pub use super::cube::{CubeType, CubeMeta, Cube};
pub use super::{Vessel, EntityFlow};


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
    pub fn vec_all() -> Vec<Self> {
        use Router::*;
        vec! [
            // BirdView,
            Workspace,
            Promised,
            // Calendar,
            // TimeAnchor,
        
            Settings,
        ]
    }
}

/// A overall layer of routers and cubes.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Glass {
    #[serde(default)]
    pub router: Router,
    map: HashMap<Router, Vec<Cube>>,
    factory: CubeIdFactory,
}

impl Default for Glass {
    fn default() -> Self {
        let mut factory = CubeIdFactory::default();
        let map = Router::vec_all().iter()
            .map(|&router| {
                (router, vec![Cube::new_router(router, &mut factory)])
            }).collect();
        Self {
            router: Router::default(),
            map,
            factory
        }
    }
}

impl Glass {
    pub fn get_cube_vec(&self) -> Vec<Cube> {
        let vec = 
            self.map.get(&self.router).cloned()
            .unwrap_or(Vec::new());
        if vec.is_empty() {
            vec![Cube::default()]
        } else {
            vec
        }
    }
    pub fn get_cube(&mut self, meta: CubeMeta) -> Option<Cube> {
        let router = meta.router;
        let vec = self.ensured(router);
        let idx = meta.idx;
        vec.get(idx).cloned()
    }
    /// inserts a cube within a safe idx
    pub fn insert_cube(&mut self, cube: Cube, meta: CubeMeta) -> Result<(),()> {
        let router = meta.router;
        let vec = self.ensured(router);
        let idx = if meta.idx > vec.len() {
            return Err(())
        } else { meta.idx };
        self.map.get_mut(&router).map(|vec|{
            vec.insert(idx, cube);
        });
        Ok(())
    }
    pub fn push_cube(&mut self, cube: Cube, router: Router) -> Result<(),()> {
        let idx = self.ensured(router).len();
        let meta = CubeMeta { 
            router,
            idx,
        };
        self.insert_cube(cube, meta)
    }
    /// removes a cube within a safe idx
    pub fn remove_cube(&mut self, meta: CubeMeta) -> Result<Cube,()> {
        let router = meta.router;
        let vec = self.ensured(router);
        let idx = if meta.idx >= vec.len() {
            return Err(())
        } else { meta.idx };
        let cube = self.map.get_mut(&router).map(|vec|{
            vec.remove(idx)
        }).expect("glass.ensured() failed.");
        Ok(cube)
    }
    pub fn swap_cube(&mut self, meta_1: CubeMeta, meta_2: CubeMeta) -> Result<(),()> {
        let cube = if let Some(cube) = self.get_cube(meta_2) {
            cube
        } else { return Err(()) };
        let cube = self.replace_cube(cube, meta_1)?;
        self.replace_cube(cube, meta_2)?;
        Ok(())
    }
    /// replaces the cube at meta with a new one
    pub fn replace_cube(&mut self, cube: Cube, meta: CubeMeta) -> Result<Cube,()> {
        let cube_ = self.remove_cube(meta)?;
        self.insert_cube(cube, meta)?;
        Ok(cube_)
    }
    /// ensure and clean
    pub fn refresh(&mut self, flow: &EntityFlow) {
        let _: Vec<()> = Router::vec_all().iter().map(|&router| {
            self.ensured(router);
        }).collect();
        self.clean(flow);
    }
    fn ensured(&mut self, router: Router) -> &mut Vec<Cube> {
        self.map.entry(router).or_insert(vec![Cube::new_router(router, &mut self.factory)])
    }
    /// is_valid and is_blank check
    fn clean(&mut self, flow: &EntityFlow) {
        let mut factory = self.factory.clone();
        let _: Vec<()> = self.map.iter_mut().map(|(&router, vec)| {
            vec.retain(|x| x.is_valid(flow) );
            if vec.is_empty() {
                vec.push(Cube::new_router(router, &mut factory))
            }
        }).collect();
        self.factory = factory;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use Router::*;
    #[test]
    fn glass_update() {
        let mut glass = Glass::default();
        switch(Workspace, &mut glass);
        let mut vessel = Vessel::default();
        vessel.glass_refresh();
    }
    fn switch(router: Router, glass: &mut Glass) {
        glass.router = router;
        println!("{:#?}", glass);
        println!("{:?}: {:?}", router, glass.get_cube_vec());
    }
}
