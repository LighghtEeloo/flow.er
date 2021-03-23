use serde::{Serialize, Deserialize};
use std::{collections::HashMap, time::SystemTime};

use super::{EntityId, now};

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

/// The basic unit of view, containing minimum info for rendering.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Cube {
    /// A single entity's notebook.
    Inkblot {
        obj: EntityId
    },
    /// A single entity and its points.
    PointView {
        obj: EntityId,
        current: Option<usize>
    },
    /// A single entity and a todo list view
    TodoList {
        obj: EntityId,
        current: Option<usize>
    },
    /// A todo-oriented view accross the app
    PromisedLand,
    FlowView {
        obj: EntityId,
        current: EntityId
    },
    CalendarView {
        current: SystemTime
    },
    TimeView,
    SettingView,
    Blank {
        alt: String
    }
}

impl Default for Cube {
    fn default() -> Self {
        Cube::Blank {
            alt: format!("Focus on a node to start.")
        }
    }
}

impl Cube {
    pub fn new(router: Router) -> Self {
        use Cube::*;
        match router {
            Router::BirdView => 
                FlowView {
                    obj: EntityId::default(),
                    current: EntityId::default()
                },
            Router::Board => Cube::default(),
            Router::Promised => Cube::default(),
            Router::Calendar => CalendarView { current: now() },
            Router::TimeAnchor => TimeView,
            Router::Settings => SettingView,
        }
    }
    pub fn is_blank(&self) -> bool {
        if let Cube::Blank{ alt: _ } = self {
            true
        } else {
            false
        }
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
    pub fn switch_router(&self, router: Router) -> Vec<Cube> {
        let vec = 
            self.map.get(&router).cloned()
            .unwrap_or(Vec::new());
        if vec.is_empty() {
            vec![Cube::default()]
        } else {
            vec
        }
    }
    pub fn refresh(&mut self) {
        let _: Vec<()> = Router::vec_all().iter().map(|&router| {
            self.ensured(router);
        }).collect();
        self.clean();
    }
    fn ensured(&mut self, router: Router) -> Vec<Cube> {
        // let mut maybe_vec = self.map.get(&router).cloned();
        // if maybe_vec.is_none() {
        //     self.map.insert(router, vec![Cube::new(router)]);
        //     maybe_vec = self.map.get(&router).cloned();
        // }
        // maybe_vec.expect("should have inserted before")
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
        println!("{:?}: {:?}", router, glass.switch_router(router));
    }
}
