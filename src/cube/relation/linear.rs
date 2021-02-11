use crate::util::*;
use crate::cube::prelude::*;

// Linear

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct LinearModel {
    pub data: Vec<EntryId>,
    pub pos: Option<usize>,
    pub fix: Fix,
}

impl LinearModel {
    pub fn new() -> Self {
        LinearModel::default()
    }
    fn locate(&self, target: EntryId) -> Option<usize> {
        self.data.iter().position(|&x| target == x)
    }
    fn try_move(&mut self, delta: isize) {
        match self.pos {
            Some(pos) => {
                let pos = pos as isize + delta;
                let pos: usize = if pos < 0 { 0 } else if pos < self.data.len() as isize { pos as usize } else { self.data.len() - 1 };
                self.pos = Some(pos);
            }
            None => ()
        }
    }
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub enum Fix {
    Deactivated,
    Above,
    Mid,
    Below,
}

impl Fix {
    fn translate(&mut self, dir: Direction) -> isize {
        use Fix::*;
        use Direction::*;
        let (mut next, i) = match self {
            Above => match dir {
                Up => (Below, 2),
                Down => (Mid, 1),
                _ => (Mid, 0)
            }
            Mid => match dir {
                Up => (Above, -1),
                Down => (Below, 1),
                _ => (Mid, 0)
            }
            Below => match dir {
                Up => (Mid, -1),
                Down => (Above, -2),
                _ => (Mid, 0)
            }
            _ => (Deactivated, 0)
        };
        mem::swap(&mut next, self);
        i
    }
    fn activate(&mut self) {
        use Fix::*;
        let mut next = match self {
            Deactivated => Mid,
            _ => self.clone()
        };
        mem::swap(&mut next, self);
    }
}

impl RelationModel for LinearModel {
    fn add(&mut self, target: EntryId, des: Option<EntryId>) {
        let pos = 
            match des {
                Some(des) => {
                    if let Some(pos) = self.locate(des) {
                        pos + 1
                    } else {
                        self.data.len()
                    }
                }
                None => 0
            };
        self.data.insert(pos, target);
        self.pos = Some(pos);
        self.fix = Fix::Deactivated;
    }
    fn del(&mut self, target: EntryId) {
        // Debug..
        LOG!("Deleting: {:?}", self.pos);
        match self.locate(target) {
            Some(pos) => {
                self.data.remove(pos);
                self.pos = None;
            }
            None => ()
        };
    }
    fn current(&self) -> Option<EntryId> {
        match self.pos {
            Some(pos) => Some(self.data[pos]),
            None => None
        }
    }
    fn focus(&mut self, target: EntryId) {
        self.pos = self.locate(target)
    }
    fn wander(&mut self, dir: Direction, fixed: bool) {
        use Fix::*;
        if self.data.len() == 0 { return }
        let delta: isize = match dir {
            Direction::Up => -1,
            Direction::Stay => 0,
            Direction::Down => 1
        };
        if fixed {
            // Todo: around fix: use a state-style fix.
            self.fix.activate();
            let will_move = match self.pos {
                Some(0) => Direction::Down == dir,
                Some(x) => {
                    x != self.data.len() - 1 || Direction::Up == dir
                }
                None => false,
                _ => true
            };
            if will_move {
                let delta = self.fix.translate(dir);
                self.try_move(delta);
            }

        } else {
            self.fix = Deactivated;
            self.pos = match self.pos {
                Some(pos) => {
                    let pos = pos as isize + delta;
                    if pos < 0 {
                        None
                    } else if pos < self.data.len() as isize {
                        Some(pos as usize)
                    } else {
                        Some(self.data.len() - 1)
                    }
                }
                None => {
                    match dir {
                        Direction::Up => None,
                        Direction::Stay => None,
                        Direction::Down => Some(0)
                    }
                }
            };
        }


    }
    fn clear(&mut self) {
        mem::take(self);
    }
}

impl Default for LinearModel {
    fn default() -> Self {
        Self {
            data: Vec::new(),
            pos: None,
            fix: Fix::Deactivated,
        }
    }
}
