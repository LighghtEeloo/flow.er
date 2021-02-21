use crate::util::*;
use super::prelude::*;

/// Flow
/// The base model for flow-er. It (as well as Linear and Graph) holds only the entity-ids
/// and search for the actual entity when needed. It can have multiple roots and can
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Flow<Id> 
where Id: Identity
{
    pub map: HashMap<Id, FlowNode<Id>>,
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
    /// Add position: with target & dir. 
    /// Descend: obj.owner = target       and obj in target.descendants; 
    /// Ascend:  obj.owner = target.owner and obj in obj.owner.descendant, 
    ///          target.owner = obj       and target in obj.descendants;
    /// _: Descend target.
    /// if None is presented, create new root.
    fn add(&mut self, obj: Id, target: Option<Id>, dir: Direction) {
        let owner: Option<Id> = target.map(|x| match dir {
            Direction::Ascend => self.map.get(&x).map(|x| x.owner).flatten(),
            _ => target
        }).flatten().filter(|x| {
            self.map.get(&x).is_none()
        });
        match owner {
            Some(id) => {
                self.map.insert(obj, FlowNode::new(Some(id)));
                self.map.get_mut(&id).expect("no owner found").descendant.push(obj);
                match dir {
                    Direction::Ascend => {
                        let obj_node = self.map.get_mut(&obj).expect("failed insert obj");
                        let des = target.expect("Ascend when None as des");
                        obj_node.descendant.push(des);
                        for x in obj_node.descendant.clone() {
                            self.map.get_mut(&x).expect("Ascend when des is not found").owner = Some(obj);
                        }
                    }
                    _ => ()
                }
            }
            None => {
                // Always a new roots if None = des && None = self.pos
                self.map.insert(obj, FlowNode::new(None));
                self.roots.push(obj);
            }
        }
        self.focus(obj);
    }
    /// Recursively delete the node and all its descendants. 
    /// self.pos = owner
    fn del_recursive(&mut self, obj: Id) {
        let node = self.map.remove(&obj).unwrap_or(FlowNode::default());
        let mut descendant = node.descendant;
        // recursively del
        loop {
            let mut collector = vec![];
            mem::swap(&mut collector, &mut descendant);
            for x in collector {
                let descendant_ = 
                    self.map.remove(&x).map(|n| n.descendant).unwrap_or(vec![]);
                descendant.extend(descendant_);
            }
            if descendant.is_empty() { break }
        }
        // trim to be valid.
        self.trim();
        self.pos = node.owner;
    }
    // fn get(&self, id: &Id) -> &FlowNode<Id> {
    //     self.map.get(id).expect("FlowNode of the required id not found.")
    // }
    // fn get_mut(&mut self, id: &Id) -> &mut FlowNode<Id> {
    //     self.map.get_mut(id).expect("FlowNode of the required id not found.")
    // }
    fn trim(&mut self) {
        let exist: HashSet<Id> = self.map.keys().cloned().collect();
        self.roots = self.roots.iter().cloned().filter(|x| exist.get(&x).is_some()).collect();
        // Todo: trim map -> descendant.
    }
    // /// move according to number delta, and return true if moved
    // fn try_move(&mut self, delta: isize) -> isize {
    //     match self.pos {
    //         Some(pos) => {
    //             let pos_ = pos as isize + delta;
    //             let pos_: usize = if pos_ < 0 { 0 } else if pos_ < self.map.len() as isize { pos_ as usize } else { self.map.len() - 1 };
    //             self.pos = Some(pos_);
    //             pos_ as isize - pos as isize
    //         }
    //         None => 0
    //     }
    // }
    fn clear(&mut self) {
        mem::take(self);
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct FlowNode<Id> 
where Id: Identity
{
    pub descendant: Vec<Id>,
    /// owner:  
    /// Some(id) if there is one;  
    /// None if roots.
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


impl<Id> Dancer<Id> for Flow<Id> 
where Id: Identity
{
    /// Return the current pos in Flow.
    fn current(&self) -> Option<Id> {
        match self.pos {
            Some(obj) => Some(obj),
            None => None
        }
    }
    fn focus(&mut self, obj: Id) {
        // validate obj.
        if let Some(_) = self.map.get(&obj) {
            self.pos = Some(obj)
        }
    }
    fn wander(&mut self, dir: Direction, fixed: bool) {
        use FixState::*;
        if self.map.len() == 0 { return }
        if Direction::Stay == dir && fixed == false {
            self.fix.deactivate();
            return
        }
        // let delta = dir.translate();
        if fixed {
        //     self.fix.activate();
        //     let delta = self.fix.translate(dir);
        //     let delta = self.try_move(delta);
        //     self.fix.shift_delta(delta);
        } else {
        //     self.fix.deactivate();
        //     self.pos = match self.pos {
        //         Some(pos) => {
        //             let pos = pos as isize + delta;
        //             if pos < 0 {
        //                 None
        //             } else if pos < self.map.len() as isize {
        //                 Some(pos as usize)
        //             } else {
        //                 Some(self.map.len() - 1)
        //             }
        //         }
        //         None => {
        //             match dir {
        //                 Direction::Descend => Some(0),
        //                 _ => None,
        //             }
        //         }
        //     };
        }
    }
}

impl<Id> Default for Flow<Id> 
where Id: Identity
{
    fn default() -> Self {
        Self {
            map: HashMap::new(),
            pos: None,
            roots: Vec::new(),
            fix: FixState::Deactivated,
        }
    }
}
