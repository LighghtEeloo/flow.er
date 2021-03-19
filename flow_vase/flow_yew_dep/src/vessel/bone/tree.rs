use crate::util::*;
use crate::yew_util::*;
use super::prelude::*;


/// A tree provides non-cycle tree view.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Tree<Id>
where Id: Identity
{
    pub map: HashMap<Id, FlowNode<Id>>,
    #[serde(skip)]
    refs: HashMap<Id, NodeRef>,
    /// root should be None iff map.is_empty.
    pub root: Option<Id>,
    pub pos: Option<Id>,
    pub fix: FixState<Id>,
}

impl<Id> Tree<Id>
where Id: Identity 
{
    pub fn from_flow(flow: &Flow<Id>, target: &Id) -> Self {
        let mut vec = vec![target.clone()];
        vec.extend(flow.get(target, "tree build failed").descendant.clone());
        // Todo: find nodes to include recursively
        Self {
            map: HashMap::from_iter(vec.clone().into_iter().map(|x| (x, flow.get(&x, "tree build failed").clone())) ),
            refs: HashMap::from_iter(vec.clone().into_iter().map(|x| (x, NodeRef::default())) ),
            root: Some(target.clone()),
            pos: None,
            fix: FixState::Deactivated
        }
    }
}



// Dancer

impl<Id> Dancer<Id> for Tree<Id>
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
    fn wander(&mut self, dir: Direction, fixed: bool) {
        if self.map.is_empty() { return }
        if Direction::Stay == dir && fixed == false {
            self.fix.deactivate();
            return
        }
        // Todo: migrate flow.
        if fixed {

        } else {

        }
    }
}

