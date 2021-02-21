use crate::util::*;
use super::prelude::*;

// Flow

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Flow<Id> 
where Id: Identity
{
    pub data: HashMap<Id, FlowNode<Id>>,
    pub roots: Vec<Id>,
    pub pos: Option<Id>,
    pub fix: FixState<Id>,
}

impl<Id> Flow<Id> 
where Id: Identity
{
    pub fn new() -> Self {
        Flow::default()
    }
    // fn get(&self, id: &Id) -> &FlowNode<Id> {
    //     self.data.get(id).expect("FlowNode of the required id not found.")
    // }
    // fn get_mut(&mut self, id: &Id) -> &mut FlowNode<Id> {
    //     self.data.get_mut(id).expect("FlowNode of the required id not found.")
    // }
    // pub fn add_orphan(&mut self, obj: Id) {
    //     self.data.insert(obj, FlowNode::new(None));
    //     self.roots.push(obj);
    // }
    fn trim(&mut self) {
        let exist: HashSet<Id> = self.data.keys().cloned().collect();
        self.roots = self.roots.iter().cloned().filter(|x| exist.get(&x).is_some()).collect();
        // Todo: trim data -> descendant.
    }
    // /// move according to number delta, and return true if moved
    // fn try_move(&mut self, delta: isize) -> isize {
    //     match self.pos {
    //         Some(pos) => {
    //             let pos_ = pos as isize + delta;
    //             let pos_: usize = if pos_ < 0 { 0 } else if pos_ < self.data.len() as isize { pos_ as usize } else { self.data.len() - 1 };
    //             self.pos = Some(pos_);
    //             pos_ as isize - pos as isize
    //         }
    //         None => 0
    //     }
    // }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct FlowNode<Id> 
where Id: Identity
{
    pub descendant: Vec<Id>,
    /// owner:  
    /// Some(id) if there is one;  
    /// None if roots or orphan.
    pub owner: Option<Id>,
}

impl<Id> FlowNode<Id> 
where Id: Identity
{
    fn new(elderly: Option<Id>) -> Self {
        Self {
            descendant: Vec::new(),
            owner: elderly,
        }
    }
}

impl<Id> Default for FlowNode<Id> 
where Id: Identity
{
    fn default() -> Self {
        Self::new(None)
    }
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub enum FixState<Id> {
    Deactivated,
    Descendant(Vec<Id>, usize),
    Elderly(Vec<Id>, usize),
}

impl<Id> FixState<Id> 
where Id: PartialEq + Eq
{
    // /// [translate] the movement proposal
    // fn translate(&self, dir: Direction) -> isize {
    //     use FixState::*;
    //     use Direction::*;
    //     0
    // }
    // /// perform the self [shift]
    // fn shift(&mut self, dir: Direction) {
    //     self.shift_delta(dir.translate());
    // }
    // /// perform the self [shift] with delta
    // fn shift_delta(&mut self, delta: isize) {
    //     use FixState::*;
    //     let pos: isize = match self {
    //         Relative(x) => x.clone(),
    //         _ => 0
    //     };
    //     let mut pos_new = pos + delta;
    //     pos_new += 1;
    //     pos_new = ((pos_new % 3) + 3) % 3; // modulus
    //     pos_new -= 1;
    //     let mut next = Relative(pos_new);
    //     mem::swap(&mut next, self);
    // }
    // fn activate(&mut self) {
    //     use FixState::*;
    //     let mut next = match self {
    //         Deactivated => Relative(0),
    //         _ => self.clone()
    //     };
    //     mem::swap(&mut next, self);
    // }
    fn deactivate(&mut self) {
        mem::swap(&mut FixState::Deactivated, self);
    }
}

impl<Id> Dancer<Id> for Flow<Id> 
where Id: Identity
{
    /// Add location: des > self.pos. if None is presented, create new roots;
    /// and then if Some = self.roots, descend the old roots.
    // fn add(&mut self, obj: Id, des: Option<Id>) {
    //     let des = if let None = des {
    //         self.pos
    //     } else {
    //         des
    //     };
    //     match des {
    //         Some(id) => {
    //             self.data.insert(obj, FlowNode::new(Some(id)));
    //             self.get_mut(&id).descendant.push(obj);
    //         }
    //         None => {
    //             // Always a new roots if None = des && None = self.pos
    //             self.data.insert(obj, FlowNode::new(None));
    //             if let Some(roots_id) = self.roots {
    //                 // Replace roots if needed
    //                 self.get_mut(&obj).descendant.push(roots_id);
    //             }
    //             self.roots = Some(obj);
    //         }
    //     }
    //     self.focus(obj);
    // }
    /// Recursively delete the node and all its descendants. 
    /// self.pos = owner
    // fn del(&mut self, obj: Id) {
    //     let node = self.data.remove(&obj).unwrap_or(FlowNode::default());
    //     let mut descendant = node.descendant;
    //     // recursively del
    //     loop {
    //         let mut collector = vec![];
    //         mem::swap(&mut collector, &mut descendant);
    //         for x in collector {
    //             let descendant_ = 
    //                 self.data.remove(&x).map(|n| n.descendant).unwrap_or(vec![]);
    //             descendant.extend(descendant_);
    //         }
    //         if descendant.is_empty() { break }
    //     }
    //     // trim to be valid.
    //     self.trim();
    //     self.pos = node.owner;
    // }
    // /// Return the current pos in Flow.
    // fn current(&self) -> Option<Id> {
    //     match self.pos {
    //         Some(obj) => Some(obj),
    //         None => None
    //     }
    // }
    // fn focus(&mut self, obj: Id) {
    //     // validate obj.
    //     if let Some(_) = self.data.get(&obj) {
    //         self.pos = Some(obj)
    //     }
    // }
    // fn wander(&mut self, dir: Direction, fixed: bool) {
    //     use FixState::*;
    //     if self.data.len() == 0 { return }
    //     if Direction::Stay == dir && fixed == false {
    //         self.fix.deactivate();
    //         return
    //     }
    //     // let delta = dir.translate();
    //     if fixed {
    //     //     self.fix.activate();
    //     //     let delta = self.fix.translate(dir);
    //     //     let delta = self.try_move(delta);
    //     //     self.fix.shift_delta(delta);
    //     } else {
    //     //     self.fix.deactivate();
    //     //     self.pos = match self.pos {
    //     //         Some(pos) => {
    //     //             let pos = pos as isize + delta;
    //     //             if pos < 0 {
    //     //                 None
    //     //             } else if pos < self.data.len() as isize {
    //     //                 Some(pos as usize)
    //     //             } else {
    //     //                 Some(self.data.len() - 1)
    //     //             }
    //     //         }
    //     //         None => {
    //     //             match dir {
    //     //                 Direction::Descend => Some(0),
    //     //                 _ => None,
    //     //             }
    //     //         }
    //     //     };
    //     }
    // }
    fn clear(&mut self) {
        mem::take(self);
    }
}

impl<Id> Default for Flow<Id> 
where Id: Identity
{
    fn default() -> Self {
        Self {
            data: HashMap::new(),
            pos: None,
            roots: Vec::new(),
            fix: FixState::Deactivated,
        }
    }
}
