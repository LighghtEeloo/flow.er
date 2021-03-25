use serde::{Serialize, Deserialize};
use std::{collections::HashMap};

pub use super::cube::{CubeType, CubeMeta, Cube};

// Todo: blob export.
pub mod cubes {
    pub use crate::cube::{
        Inkblot,
        ClauseTree,
        PromisedLand,
        FlowView,
        CalendarView,
        TimeView,
        SettingView,
        Blank
    };
}

/// Describes the app router.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Deserialize, Serialize)]
pub enum Router {
    BirdView,
    Board,
    Promised,
    Calendar,
    TimeAnchor,

    Settings,
}

impl Default for Router {
    fn default() -> Self {
        Router::Board
    }
}

impl Router {
    pub fn type_str(&self) -> &'static str {
        use Router::*;
        match self {
            BirdView => "bird-view",
            Board => "board",
            Promised => "promised",
            Calendar => "calendar",
            TimeAnchor => "time-capsule",
            Settings => "settings"
        }
    }
    pub fn display_str(&self) -> &'static str {
        use Router::*;
        match self {
            BirdView => "BirdView",
            Board => "Board",
            Promised => "Promised",
            Calendar => "Calendar",
            TimeAnchor => "TimeAnchor",
            Settings => "Settings"
        }
    }
    pub fn vec_all() -> Vec<Self> {
        use Router::*;
        vec! [
            BirdView,
            Board,
            Promised,
            Calendar,
            TimeAnchor,
        
            Settings,
        ]
    }
}

/// A overall layer of routers and cubes.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Glass {
    map: HashMap<Router, Vec<Cube>>
}

impl Default for Glass {
    fn default() -> Self {
        let map = Router::vec_all().iter()
            .map(|&router| {
                (router, vec![Cube::new(router)])
            }).collect();
        Self {
            map
        }
    }
}

impl Glass {
    pub fn get_cube_vec(&self, router: Router) -> Vec<Cube> {
        let vec = 
            self.map.get(&router).cloned()
            .unwrap_or(Vec::new());
        if vec.is_empty() {
            vec![Cube::default()]
        } else {
            vec
        }
    }
    /// inserts a cube within a safe idx
    pub fn insert_cube(&mut self, cube: Cube, meta: CubeMeta) {
        let router = meta.router;
        let vec = self.ensured(router);
        let idx = meta.idx.min(vec.len());
        self.map.get_mut(&router).map(|vec|{
            vec.insert(idx, cube);
        });
    }
    pub fn push_cube(&mut self, cube: Cube, router: Router) {
        let vec = self.ensured(router);
        let idx = vec.len();
        self.map.get_mut(&router).map(|vec|{
            vec.insert(idx, cube);
        });
    }
    /// removes a cube within a safe idx
    pub fn remove_cube(&mut self, meta: CubeMeta) -> Cube {
        let router = meta.router;
        let vec = self.ensured(router);
        let idx = meta.idx.min(vec.len());
        self.map.get_mut(&router).map(|vec|{
            vec.remove(idx)
        }).expect("glass.ensured() failed.")
    }
    pub fn swap_cube(&mut self, meta_1: CubeMeta, meta_2: CubeMeta) {
        let cube = self.remove_cube(meta_2);
        self.insert_cube(cube, meta_1);
    }
    /// replaces the cube at meta with a new one
    pub fn replace_cube(&mut self, cube: Cube, meta: CubeMeta) {
        self.remove_cube(meta);
        self.insert_cube(cube, meta)
    }
    /// ensure and clean
    pub fn refresh(&mut self) {
        let _: Vec<()> = Router::vec_all().iter().map(|&router| {
            self.ensured(router);
        }).collect();
        self.clean();
    }
    fn ensured(&mut self, router: Router) -> Vec<Cube> {
        self.map.entry(router).or_insert(vec![Cube::new(router)]).clone()
    }
    fn clean(&mut self) {
        let _: Vec<()> = self.map.iter_mut().map(|(&router, vec)| {
            vec.retain(|x| !x.is_blank() );
            if vec.is_empty() {
                vec.push(Cube::new(router))
            }
        }).collect();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use Router::*;
    #[test]
    fn glass_update() {
        let mut glass = Glass::default();
        switch(Board, &glass);
        glass.clean();
        switch(Board, &glass);
        glass.refresh();
        switch(Board, &glass);
    }
    fn switch(router: Router, glass: &Glass) {
        println!("{:#?}", glass);
        println!("{:?}: {:?}", router, glass.get_cube_vec(router));
    }
}
