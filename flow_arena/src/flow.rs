use std::{collections::HashSet, hash::Hash};

pub trait Node<Id> {
    fn id(&self) -> &Id;
    fn parent(&self) -> Option<Id>;
    fn parent_set(&mut self, id: Id);
    fn parent_set_none(&mut self);
    fn children(&self) -> Vec<Id>;
    fn children_ref_mut(&mut self) -> &mut Vec<Id>;
}

/// no check in FlowBase
pub trait FlowBase {
    type Id: Default + Clone + Hash + Eq;
    type Node: Default + Clone + Node<Self::Id>;
    /// ensures root and returns it
    fn orphan(&self) -> Vec<Self::Id>;
    fn contains_node(&self, obj: &Self::Id) -> bool {
        self.node(obj).is_some()
    }
    fn node(&self, obj: &Self::Id) -> Option<&Self::Node>;
    fn node_mut(&mut self, obj: &Self::Id) -> Option<&mut Self::Node>;
    fn parent(&self, obj: &Self::Id) -> Option<Self::Id> {
        self.node(obj).map_or(None, 
        |node| {
            node.parent()
        })
    }
    fn children(&self, obj: &Self::Id) -> Vec<Self::Id> {
        self.node(obj).map_or(Vec::new(), 
        |node| {
            node.children()
        })
    }

    /// returns parent's children
    fn friends(&self, obj: &Self::Id) -> Vec<Self::Id> {
        self.parent(obj).map_or(Vec::new(), |obj| {
            self.children(&obj)
        })
    }

    /// returns nth position in friends; None if not found in either way
    fn nth_friend(&self, obj: &Self::Id) -> Option<usize> {
        self.friends(obj).into_iter()
        .position(|id| &id == obj)
    }

    /// returns owned children
    fn children_owned(&self, obj: &Self::Id) -> Vec<Self::Id> {
        self.children(obj).into_iter().filter_map(|id| {
            if self.parent(&id) == Some(obj.clone()) {
                Some(id)
            } else {
                None
            }
        }).collect()
    }

    /// judge whether the obj is owned by the owner
    fn is_owned(&self, obj: &Self::Id, owner: &Self::Id) -> bool {
        self.parent(obj).map_or(false, |id| &id == owner)
        && self.children(owner).contains(obj)
    }

    /// judge whether the obj is *purely* linked from the owner
    fn is_linked(&self, obj: &Self::Id, owner: &Self::Id) -> bool {
        self.parent(obj).map_or(true, |id| &id != owner)
        && self.children(owner).contains(obj)
    }

    /// returns all the offspring of a node, not including itself
    fn node_offspring_set(&self, obj: &Self::Id) -> HashSet<Self::Id> {
        let mut visit_set = HashSet::new();
        let mut final_set = HashSet::new();
        visit_set.insert(obj.clone());
        while !visit_set.is_empty() {
            let mut wait_set = HashSet::new();
            for obj in visit_set.iter() {
                let children = self.node(&obj)
                    .map(|x| x.children() )
                    .unwrap_or_default();
                wait_set.extend(children);
            }
            final_set.extend(wait_set.iter().cloned());
            visit_set.clear();
            visit_set.extend(wait_set);
        }
        final_set
    }

    /// returns all the nodes owned by a node, including itself
    fn node_ownership_set(&self, obj: &Self::Id) -> HashSet<Self::Id> {
        let mut visit_set = HashSet::new();
        let mut final_set = HashSet::new();
        if self.contains_node(obj) {
            visit_set.insert(obj.clone());
            final_set.insert(obj.clone());
        }
        while !visit_set.is_empty() {
            let mut wait_set = HashSet::new();
            for obj in visit_set.iter() {
                let children = self.node(&obj)
                    .map(|x|  x.children())
                    .unwrap_or_default();
                let set: Vec<Self::Id> = children.into_iter().filter_map(|id| {
                    self.node(&id).map(|node| {
                        if node.parent() == Some(obj.clone()) {
                            Some(id)
                        } else {
                            None
                        }
                    }).flatten()
                }).collect();
                wait_set.extend(set);
            }
            final_set.extend(wait_set.iter().cloned());
            visit_set.clear();
            visit_set.extend(wait_set);
        }
        final_set
    }
}

pub trait FlowCheck {

    fn check(&self) -> Result<(), (FlowError, String)>;
        /// panics if anything went wrong. Iff in debug state.
    fn check_assert(&self) {
        if cfg!(debug_assertions) {
            if let Err((err, current)) = self.check() {
                panic!("{:?}{}", err, current)
            }   
        } 
    } 
}

pub trait FlowLink: FlowBase + FlowCheck {
    /// randomly ensures the link of a node to another
    fn link(&mut self, obj: &Self::Id, owner: &Self::Id, nth: usize) -> Result<(), FlowError> {
        if ! self.contains_node(obj) {
            Err(FlowError::NotExistObj)?
        } 
        let res = self.node_mut(owner).map(|owner| {
            if owner.children().contains(obj) {
                return Ok(())
            } 
            if nth > owner.children().len() {
                Err(FlowError::InvalidLen)?
            } 
            owner.children_ref_mut().insert(nth, obj.clone());
            Ok(())
        }).unwrap_or(Err(FlowError::NotExistOwner));
        self.check_assert();
        res
    }

    fn link_push(&mut self, obj: &Self::Id, owner: &Self::Id) -> Result<(), FlowError> {
        let nth = self.node(owner).map_or(0, |node| node.children().len());
        let res = self.link(obj, owner, nth);
        self.check_assert();
        res
    }
    /// detaches a node from a non-owner link
    fn detach(&mut self, obj: &Self::Id, owner: &Self::Id) -> Result<(), FlowError> {
        if ! self.contains_node(obj) {
            Err(FlowError::NotExistObj)?
        } 
        let res = self.node_mut(owner).map(|owner| {
            if ! owner.children().contains(obj) {
                Err(FlowError::AbandonedChild)?
            }
            owner.children_ref_mut().retain(|x| x != obj);
            Ok(())
        }).unwrap_or(Err(FlowError::NotExistOwner));
        self.check_assert();
        res
    }
}

pub trait FlowMaid: FlowBase + FlowLink + FlowCheck {
    /// inserts a node; returns err if id exists.
    fn grow(&mut self, obj: Self::Node) -> Result<Self::Id, FlowError>;

    /// appoints and ensures an owner; also links to owner
    fn devote(&mut self, obj: &Self::Id, owner: &Self::Id, nth: usize) -> Result<(), FlowError> {
        let res = self.link(obj, owner, nth).and_then(|_| {
            self.node_mut(obj).map(|obj| {
                obj.parent_set(owner.clone());
                Ok(())
            }).unwrap_or(Err(FlowError::NotExistObj))
        });
        self.check_assert();
        res
    }

    fn devote_push(&mut self, obj: &Self::Id, owner: &Self::Id) -> Result<(), FlowError> {
        let res = self.link_push(obj, owner).and_then(|_| {
            self.node_mut(obj).map_or(
                Err(FlowError::NotExistObj), 
                |obj| {
                    obj.parent_set(owner.clone());
                    Ok(())
                }
            )
        });
        self.check_assert();
        res
    }

    /// removes ownership; also detaches
    fn decay(&mut self, obj: &Self::Id) -> Result<(), FlowError> {
        let owner = self.node(obj)
            .map(|x| {
                x.parent() 
            });
        // Not owned by anyone
        if let Some(None) = owner {
            return Ok(())
        }
        let owner = owner.flatten();
        let res = self.node_mut(obj).map_or(
            Err(FlowError::NotExistObj),
            |obj| {
                obj.parent_set_none();
                Ok(())
            }
        ).and_then(|_| {
            owner.map_or(
                Err(FlowError::NotExistOwner), 
                |owner| {
                    self.detach(obj, &owner)
                }
            )
        });
        self.check_assert();
        res
    }

    /// decay before devote
    fn devote_loyal(&mut self, obj: &Self::Id, owner: &Self::Id, nth: usize) -> Result<(), FlowError> {
        self.decay(&obj)?;
        self.devote(&obj, &owner, nth)
    }

    /// decay before devote
    fn devote_loyal_push(&mut self, obj: &Self::Id, owner: &Self::Id) -> Result<(), FlowError> {
        self.decay(&obj)?;
        self.devote_push(&obj, &owner)
    }

    /// removes a node; returns err if id not found under root
    fn erase(&mut self, obj: &Self::Id) -> Result<(), FlowError>;
}

pub trait FlowDock: FlowMaid + FlowCheck + Sized {
    /// adds all the nodes in another flow to self and mounts all orphan nodes to the designated node
    ///
    /// Err if:
    /// 1. Owner not found.
    /// 2. Node exists in current flow.
    fn dock_unordered(&mut self, owner: &Self::Id, flow: Self) -> Result<(), FlowError> {
        self.dock(owner, flow.orphan(), flow)
    }
    fn dock(&mut self, owner: &Self::Id, vec: Vec<Self::Id>, flow: Self) -> Result<(), FlowError>;
    /// moves all the nodes under the designated node out of the current flow and unmounts them
    /// 
    /// Err if:
    /// 1. Obj not found.
    /// 2. Node linked by other nodes.
    fn undock_impl(&mut self, obj: &Self::Id, owned: bool) -> Result<(Self, Vec<Self::Id>), FlowError>;
    fn undock(&mut self, obj: &Self::Id) -> Result<(Self, Vec<Self::Id>), FlowError> {
        self.undock_impl(obj, false)
    }
    fn undock_owned(&mut self, obj: &Self::Id) -> Result<(Self, Vec<Self::Id>), FlowError> {
        self.undock_impl(obj, true)
    }
    /// clones all the nodes linked under the designated node and unmounts the clone
    /// 
    /// Err if:
    /// 1. Obj not found.
    fn snap(&self, obj: &Self::Id) -> Result<(Self, Vec<Self::Id>), FlowError>;
    /// clones all the nodes owned under the designated node and unmounts the clone
    /// 
    /// Err if:
    /// 1. Obj not found.
    fn snap_owned(&self, obj: &Self::Id) -> Result<(Self, Vec<Self::Id>), FlowError>;
}

#[derive(Debug, Copy, Clone)]
pub enum Direction {
    Forward,
    Backward,
    Ascend,
    Descend,
}

impl Direction {
    fn shift(&self) -> isize {
        use Direction::*;
        match self {
            Forward => 1,
            Backward => -1,
            _ => 0
        }
    }

    /// bounded in [0, len)
    fn walk(&self, nth: usize, len: usize) -> Result<usize,()> {
        let pos = nth as isize + self.shift();
        if pos < 0 {
            Err(())
        } else if (pos as usize) < len {
            Ok(pos as usize)
        } else {
            Err(())
        }
        
    }
}

pub trait FlowShift: FlowBase + FlowMaid {
    /// returns the obj in the corresponding relative position
    fn shuttle(&self, obj: &Self::Id, dir: Direction) -> Result<Self::Id, FlowError> {
        use Direction::*;
        match dir {
            Forward | Backward => {
                let friends = self.friends(obj);
                let nth = if let Some(nth) = friends.iter()
                    .position(|id| {
                        id == obj
                    }) 
                { nth } else { Err(FlowError::AbandonedChild)? };
                let len = friends.len();
                let walk = dir.walk(nth, len)
                    .map_err(|_| FlowError::InvalidLen)?;
                Ok(friends[walk].clone())
            }
            Ascend => {
                let candidate = self.parent(obj);
                let res = candidate
                    .map_or(obj.clone(), |id| {
                        id
                    });
                Ok(res)
            }
            Descend => {
                let candidate = self.children(obj).get(0).cloned();
                let res = candidate
                    .map_or(obj.clone(), |id| {
                        id
                    });
                Ok(res)
            }
        }
    }
    fn shuttle_iter(&self, obj: &Self::Id, dir: Direction) -> Result<Self::Id, FlowError> {
        use Direction::*;
        match dir {
            Forward | Backward => {
                self.shuttle(obj, dir).map_or_else(
                    |_| {
                        Ok(self.parent(obj).unwrap_or(obj.clone()))
                    }, 
                    |id| Ok(id)
                )
            }
            Ascend | Descend => self.shuttle(obj, dir),
        }
    }

    /// alters the node position by the corresponding relative position, within a single node
    fn migrate(&mut self, obj: &Self::Id, dir: Direction) -> Result<(), FlowError> {
        use Direction::*;
        if ! self.contains_node(obj) { Err(FlowError::NotExistObj)? }
        match dir {
            Forward | Backward => {
                let owner = self.parent(obj).ok_or(FlowError::NotExistObj)?;
                let nth = self.nth_friend(obj).ok_or(FlowError::AbandonedChild)?;
                let len = self.friends(&owner).len();
                let walk = dir.walk(nth, len)
                    .map_err(|_| FlowError::InvalidLen)?;
                self.decay(obj)?;
                self.devote(obj, &owner, walk)?
            }
            Ascend => { 
                let parent = self.parent(obj).ok_or(FlowError::NotExistObj)?;
                let owner = self.parent(&parent).ok_or(FlowError::IsOrphaned)?;
                let nth = self.nth_friend(&parent).ok_or(FlowError::AbandonedChild)? + 1;
                self.decay(obj)?;
                self.devote(obj, &owner, nth)?
            }
            Descend => { Err(FlowError::InvalidDir)? }
        }
        self.check_assert();
        Ok(())
    }

    /// alters the node position by the corresponding relative position, iteratively within the flow
    fn migrate_iter(&mut self, obj: &Self::Id, dir: Direction) -> Result<(), FlowError> {
        use Direction::*;
        match dir {
            Forward | Backward => {
                self.migrate(obj, dir).map_or_else(
                    |e| {
                        if let FlowError::InvalidLen = e {
                            self.migrate(obj, Ascend)
                        } else { Err(e) }
                    }, 
                    |v| Ok(v)
                )
            }
            Ascend | Descend => self.migrate(obj, dir)
        }
    }
}

#[derive(Debug)]
pub enum FlowError {
    NotExistObj,
    NotExistOwner,
    InvalidLen,
    ExistGrow,
    ExistDock,
    OwnerDetach,
    LinkedUndock,
    InvalidDir,
    /// certain operations requires node to be orphaned
    NotOrphaned,
    /// certain operations requires node to be unorphaned
    IsOrphaned,
    
    NodeIdNotMatch,
    NotExistParent,
    NotExistChild,
    /// potential parent doesn't have the child
    AbandonedChild,
}

pub trait Flow: FlowBase + FlowLink + FlowMaid + FlowDock + FlowShift {}
