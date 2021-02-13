use crate::util::*;
use crate::cube::prelude::*;

// Linear

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct LinearModel<Id> 
{
    pub data: Vec<Id>,
    pub pos: Option<usize>,
    pub fix: FixState,
}

impl<Id> LinearModel<Id> 
where Id: Identity
{
    pub fn new() -> Self {
        LinearModel::default()
    }
    fn locate(&self, obj: Id) -> Option<usize> {
        self.data.iter().position(|&x| obj == x)
    }
    /// move according to number delta, and return true if moved
    fn try_move(&mut self, delta: isize) -> isize {
        match self.pos {
            Some(pos) => {
                let pos_ = pos as isize + delta;
                let pos_: usize = if pos_ < 0 { 0 } else if pos_ < self.data.len() as isize { pos_ as usize } else { self.data.len() - 1 };
                self.pos = Some(pos_);
                pos_ as isize - pos as isize
            }
            None => 0
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Deserialize, Serialize)]
pub enum FixState {
    Deactivated,
    Relative(isize)
}

impl FixState {
    /// [translate] the movement proposal
    fn translate(&self, dir: Direction) -> isize {
        use FixState::*;
        use Direction::*;
        match self {
            Relative(-1) => match dir {
                Up => 2,
                Down => 1,
                _ => 0
            }
            Relative(0) => match dir {
                Up => -1,
                Down => 1,
                _ => 0
            }
            Relative(1) => match dir {
                Up => -1,
                Down => -2,
                _ => 0
            }
            _ => 0
        }
    }
    /// perform the self [shift]
    fn shift(&mut self, dir: Direction) {
        self.shift_delta(dir.translate());
    }
    /// perform the self [shift] with delta
    fn shift_delta(&mut self, delta: isize) {
        use FixState::*;
        let pos: isize = match self {
            Relative(x) => x.clone(),
            _ => 0
        };
        let mut pos_new = pos + delta;
        pos_new += 1;
        pos_new = ((pos_new % 3) + 3) % 3; // modulus
        pos_new -= 1;
        let mut next = Relative(pos_new);
        mem::swap(&mut next, self);
    }
    fn activate(&mut self) {
        use FixState::*;
        let mut next = match self {
            Deactivated => Relative(0),
            _ => self.clone()
        };
        mem::swap(&mut next, self);
    }
    fn deactivate(&mut self) {
        mem::swap(&mut FixState::Deactivated, self);
    }
}

impl<Id> RelationModel<Id> for LinearModel<Id> 
where Id: Identity
{
    fn add(&mut self, obj: Id, des: Option<Id>) {
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
        self.data.insert(pos, obj);
        self.pos = Some(pos);
        self.fix = FixState::Deactivated;
    }
    fn del(&mut self, obj: Id) {
        match self.locate(obj) {
            Some(pos) => {
                self.data.remove(pos);
                self.pos = None;
            }
            None => ()
        };
    }
    fn current(&self) -> Option<Id> {
        match self.pos {
            Some(pos) => Some(self.data[pos]),
            None => None
        }
    }
    fn focus(&mut self, obj: Id) {
        self.pos = self.locate(obj)
    }
    fn wander(&mut self, dir: Direction, fixed: bool) {
        use FixState::*;
        if self.data.len() == 0 { return }
        if Direction::Stay == dir && fixed == false {
            self.fix.deactivate();
            return
        }
        let delta = dir.translate();
        if fixed {
            self.fix.activate();
            let delta = self.fix.translate(dir);
            let delta = self.try_move(delta);
            self.fix.shift_delta(delta);
        } else {
            self.fix.deactivate();
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
                        Direction::Down => Some(0),
                        _ => None,
                    }
                }
            };
        }


    }
    fn clear(&mut self) {
        mem::take(self);
    }
}

impl<Id> Default for LinearModel<Id> 
where Id: Identity
{
    fn default() -> Self {
        Self {
            data: Vec::new(),
            pos: None,
            fix: FixState::Deactivated,
        }
    }
}
