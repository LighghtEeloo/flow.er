use crate::util::*;
use super::prelude::*;

/// Flow
/// The base model for flow-er. It (as well as Linear and Tree) holds only the entity-ids
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
    pub fn add_with_link(&mut self, obj: Id, flow_link: FlowLink<Id>) {
        let target = flow_link.target;
        let dir = flow_link.dir;
        let idx = flow_link.idx;
        let owner: Option<Id> = target.map(|x| match dir {
            Direction::Ascend => self.map.get(&x).map(|x| x.owner).flatten(),
            _ => target
        }).flatten().filter(|x| {
            self.check(&x).is_ok()
        });
        match owner {
            Some(id) => {
                self.map.insert(obj, FlowNode::from_owner(Some(id)));
                let owner_node = self.get_mut(&id, "no owner found");
                idx.inserted_with(owner_node, obj);
                match dir {
                    Direction::Ascend => {
                        let obj_node = self.get_mut(&obj, "failed insert obj");
                        let des = target.expect("Ascend when None as des");
                        idx.inserted_with(obj_node, des);
                        for x in obj_node.descendant.clone() {
                            self.get_mut(&x, "Ascend when des is not found").owner = Some(obj);
                        }
                    }
                    _ => ()
                }
            }
            None => {
                // Always a new roots if None = des && None = self.pos
                self.map.insert(obj, FlowNode::from_owner(None));
                self.roots.push(obj);
            }
        }
        self.focus(obj);
    }
    // /// Recursively delete the node and all its descendants. 
    // /// self.pos = owner
    // pub fn del_recursive(&mut self, obj: Id) {
    //     let node = self.map.remove(&obj).unwrap_or(FlowNode::default());
    //     let mut descendant = node.descendant;
    //     // recursively del
    //     loop {
    //         let mut collector = vec![];
    //         mem::swap(&mut collector, &mut descendant);
    //         for x in collector {
    //             let descendant_ = 
    //                 self.map.remove(&x).map(|n| n.descendant).unwrap_or(vec![]);
    //             descendant.extend(descendant_);
    //         }
    //         if descendant.is_empty() { break }
    //     }
    //     // trim to be valid.
    //     self.trim();
    //     self.pos = node.owner.map(|x| self.check(&x).ok()).flatten();
    //     self.fix = FixState::Deactivated;
    // }
    pub fn get(&self, id: &Id, expect: &'static str) -> &FlowNode<Id> {
        self.map.get(id).expect(expect)
    }
    fn get_mut(&mut self, id: &Id, expect: &'static str) -> &mut FlowNode<Id> {
        self.map.get_mut(id).expect(expect)
    }
    /// For id, returns [owner] if id in owner.descendant;
    /// returns [] if id is root.
    /// 
    /// Note that the current impl is a bit expensive.
    fn get_owners(&self, id: &Id) -> Vec<Id> {
        match self.get(id, "invalid pos").owner {
            None => vec![],
            _ => {
                self.map.iter().filter_map(|(&tar, node)| { if node.descendant.contains(id) { Some(tar) } else { None } }).collect()
            }
        }
    }
    /// Expensive way to check and clean all nodes that are invalid.
    /// returns Err(vec) if nodes with id in vec are cleaned.
    /// 
    /// Depends on check_flow() and clean_flow().
    pub fn trim(&mut self) -> Result<(), Vec<Id>> {
        let trim_list = self.check_flow();
        if trim_list.is_empty() {
            Ok(())
        } else {
            self.clean_flow(&trim_list);
            Err(trim_list)
        }
    }
    fn check_flow(&self) -> Vec<Id> {
        let mut sift = HashSet::new();
        // Get all possible descendants
        sift.extend(self.map.iter().map(|(_, node)| {
            node.descendant.iter()
        }).flatten());
        // Get all possible owners
        sift.extend(self.map.iter().filter_map(|(_, node)| {
            node.owner
        }));
        sift.into_iter().filter(|x| self.check(x).is_err()).collect()
    }
    fn clean_flow(&mut self, list: &Vec<Id>) {
        // remove from self.map
        let _: Vec<()> = 
            list.iter()
            .map(|x| { self.map.remove(x); })
            .collect();
        let vec_descendant: Vec<&mut Vec<Id>> = 
            self.map.iter_mut()
            .map(|(_, node)| &mut node.descendant)
            .collect();
        // remove from descendant of all nodes
        let _: Vec<()> = 
            vec_descendant.into_iter()
            .map(|det| { det.retain(|x| list.iter().find(|&y| y == x).is_none()); })
            .collect();
    }
    pub fn clear(&mut self) {
        mem::take(self);
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


// Architect

impl<Id> Architect<Id> for Flow<Id> 
where Id: Identity
{
    fn add(&mut self, obj: Id) -> Result<Id, Critic> { 
        match self.map.insert(obj.clone(), FlowNode::default()) {
            Some(_) => Err(FlowNodeExistError),
            _ => Ok(obj)
        }
    }
    fn link(&mut self, obj: Id, flow_link: FlowLink<Id>) -> Result<Id, Critic> { 
        let target = flow_link.target;
        let dir = flow_link.dir;
        let idx = flow_link.idx;
        let node_obj = self.map.get_mut(&obj).ok_or(FlowNodeNotFoundError)?;
        match target {
            Some(tar) => {
                match dir {
                    Direction::Ascend => {
                        idx.inserted_with(node_obj, tar);
                        let node_tar = self.map.get_mut(&tar).ok_or(FlowNodeNotFoundError)?;
                        node_tar.owner = Some(obj);
                    }
                    _ => {
                        node_obj.owner = Some(tar);
                        let node_tar = self.map.get_mut(&tar).ok_or(FlowNodeNotFoundError)?;
                        idx.inserted_with(node_tar, obj);
                    }
                }
            }
            None => if !self.roots.contains(&obj) {
                match idx {
                    FlowLinkIndex::Head => self.roots.insert(0, obj),
                    FlowLinkIndex::Index(i) => self.roots.insert(i, obj),
                    FlowLinkIndex::Tail => self.roots.push(obj)
                }
            } 
        }
        Ok(obj)
    }
    /// depends on self.get_owners to collect.
    fn del(&mut self, obj: Id) -> Result<(), Critic> { 
        let node_obj = self.map.remove(&obj).ok_or(FlowNodeNotFoundError)?;
        let mut err_vec: Vec<Critic> = vec![];
        err_vec.extend(
            node_obj.descendant.into_iter().map(|tar| -> Result<(), Critic> {
                let node_tar = self.map.get_mut(&tar).ok_or(FlowNodeNotFoundError)?;
                node_tar.owner = None;
                Ok(())
            }).filter_map(|x| x.err()).take(1)
        );
        err_vec.extend(
            self.get_owners(&obj).into_iter().map(|tar| -> Result<(), Critic> {
                let node_tar = self.map.get_mut(&tar).ok_or(FlowNodeNotFoundError)?;
                node_tar.descendant.retain(|&x| x != obj);
                Ok(())
            }).filter_map(|x| x.err()).take(1)
        );
        match err_vec.get(0) {
            Some(&e) => Err(e),
            _ => Ok(()) 
        }
    }
}


// Dancer

impl<Id> Dancer<Id> for Flow<Id> 
where Id: Identity
{
    fn check(&self, obj: &Id) -> Result<Id, Critic> {
        if self.map.contains_key(obj) { Ok(*obj) } else { Err(FlowNodeNotFoundError) }
    }
    fn current(&self) -> Option<Id> {
        self.pos.clone()
    }
    fn focus(&mut self, obj: Id) {
        // validate obj.
        self.check(&obj).expect("trying to focus none");
        self.pos = Some(obj)
    }
    /// (Direction::Stay, false) -> exit fix;
    /// (_, false) -> move;
    /// (_, true) -> 
    ///     last: Deactivated -> to_fix and move;
    ///     last: Descendant(_) -> roll
    ///     last: Owner(_) -> roll
    fn wander(&mut self, dir: Direction, fixed: bool) {
        // use FixState::*;
        if self.map.is_empty() { return }
        if Direction::Stay == dir && fixed == false {
            self.fix.deactivate();
            return
        }
        if fixed {
            self.activate_fix(dir);
            self.pos = self.fix.to_id().or(self.roots.get(0).cloned());
        } else {
            self.fix.deactivate();
            self.pos = match self.pos {
                Some(pos) => {
                    self.check(&pos).expect("invalid pos");
                    self.shift_dir_unfixed(dir, pos)
                }
                None => {
                    match dir {
                        Direction::Descend => self.roots.get(0).cloned(),
                        _ => None,
                    }
                }
            };
        }
    }
}



impl<Id> Flow<Id> 
where Id: Identity
{
    fn activate_fix(&mut self, dir: Direction) {
        use FixState::*;
        self.fix = match self.fix.clone() {
            Deactivated => {
                self.translate_dir_fixed(dir)
            }
            Owner(vec, idx) => {
                let len = vec.len();
                Owner(vec, (idx + dir.translate()).modulo(len as isize))
            }
            Descendant(vec, idx) => {
                let len = vec.len();
                Descendant(vec, (idx + dir.translate()).modulo(len as isize))
            }
        }
    }
    /// Translate to fix from Deactivated given a dir
    fn translate_dir_fixed(&self, dir: Direction) -> FixState<Id> {
        use FixState::*;
        let obj = match self.pos.map(|x| self.check(&x).ok()).flatten() { 
            None => return Deactivated,
            Some(pos) => pos
        };
        let node = self.get(&obj, "invalid pos");
        match dir {
            Direction::Ascend => Owner(self.get_owners(&obj), 0),
            _ => Descendant(node.descendant.clone(), 0)
        }
    }
    fn shift_dir_unfixed(&mut self, dir: Direction, pos: Id) -> Option<Id> {
        let node = self.get(&pos, "invalid pos");
        match dir {
            Direction::Ascend => {
                node.owner
            }
            _ => {
                node.descendant.get(0).cloned()
            }
        }
    }
}
