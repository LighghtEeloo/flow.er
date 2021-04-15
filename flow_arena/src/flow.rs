use std::{collections::HashSet, hash::Hash};

pub trait FlowNode<Id> {
    fn id(&self) -> &Id;
    fn parent(&self) -> Option<Id>;
    fn parent_set(&mut self, id: Id);
    fn parent_set_none(&mut self);
    fn children(&self) -> Vec<Id>;
    fn children_ref_mut(&mut self) -> &mut Vec<Id>;
}

pub trait FlowBase {
    type Id: Default + Hash + Eq + Clone;
    type Node: Default + FlowNode<Self::Id>;
    /// ensures root and returns it; no check
    fn orphan(&self) -> Vec<Self::Id>;
    /// no check hereafter
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
            return Err(FlowError::NotExistObj)
        } 
        let res = self.node_mut(owner).map(|owner| {
            if owner.children().contains(obj) {
                return Ok(())
            } 
            if nth > owner.children().len() {
                return Err(FlowError::InValidLen)
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
            return Err(FlowError::NotExistObj)
        } 
        let res = self.node_mut(owner).map(|owner| {
            if ! owner.children().contains(obj) {
                return Err(FlowError::AbandonedChild)
            }
            owner.children_ref_mut().retain(|x| x != obj);
            Ok(())
        }).unwrap_or(Err(FlowError::NotExistOwner));
        self.check_assert();
        res
    }
}

pub trait FlowMaid: FlowBase + FlowLink {
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
    /// removes a node; returns err if id not found under root
    fn erase(&mut self, obj: &Self::Id) -> Result<(), FlowError>;
}

pub trait FlowDock: FlowMaid + Sized {
    /// adds all the nodes in another flow to self and mounts all orphan nodes to the designated node
    ///
    /// Err if:
    /// 1. Owner not found.
    /// 2. Node exists in current flow.
    fn dock(&mut self, owner: &Self::Id, flow: Self) -> Result<(), FlowError>;
    /// moves all the nodes under the designated node out of the current flow and unmounts them
    /// 
    /// Err if:
    /// 1. Obj not found.
    /// 2. Node linked by other nodes.
    fn undock(&mut self, obj: &Self::Id) -> Result<Self, FlowError>;
    /// clones all the nodes under the designated node and unmounts them
    /// 
    /// Err if:
    /// 1. Obj not found.
    fn snap(&self, obj: &Self::Id) -> Result<Self, FlowError>;
}

pub enum Direction {
    Forward,
    Backward,
    Ascend,
    Descent,
}

pub trait FlowShift: FlowBase {
    /// returns the obj in the corresponding relative position
    fn shuttle(&self, obj: &Self::Id, dir: Direction) -> Result<Self::Id, FlowError>;
    /// alters the node position by the corresponding relative position, within a single node
    fn migrate(&self, obj: &Self::Id, dir: Direction) -> Result<(), FlowError>;
    /// alters the node position by the corresponding relative position, iteratively within the flow
    fn migrate_iter(&self, obj: &Self::Id, dir: Direction) -> Result<(), FlowError>;
}

#[derive(Debug)]
pub enum FlowError {
    NotExistObj,
    NotExistOwner,
    InValidLen,
    ExistGrow,
    OwnerDetach,
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

/// Flow: the underlying trait for flow.er.
/// 
/// Let's start with some key concepts.
/// 
/// A. Arena
/// 
/// An arena is a typical data structure which has:
/// 1. A map / vec to store the data.
/// 2. A relationship graph which only tracks after the keys / indices.
/// 
/// FlowArena implements an arena-like data structure, but it has integrated the data map and the relationship graph, since both of them require an Id to visit. 
/// 
// Todo..
pub trait Flow: FlowBase + FlowLink + FlowMaid + FlowDock + FlowShift {}
