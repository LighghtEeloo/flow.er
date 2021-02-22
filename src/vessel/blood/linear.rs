use crate::util::*;
use super::prelude::*;


/// Linear
/// A Visualizable model for CubeView. Designed for todo-list-ish situations.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Linear<Id>
where Id: Identity
{
    pub vec: Vec<Id>,
    pub pos: Option<Id>,
    pub fix: FixState<Id>,
}

impl<Id> Linear<Id>
where Id: Identity 
{
    pub fn from_flow(flow: &Flow<Id>, target: &Id) -> Self {
        let vec = flow.get(target, "linear build failed").descendant.clone();
        Self {
            vec,
            pos: None,
            fix: FixState::Deactivated
        }
    }
}

// Dancer

impl<Id> Dancer<Id> for Linear<Id>
where Id: Identity
{
    fn check(&self, obj: &Id) -> Result<Id, FlowNodeNotFoundError> {
        if self.vec.contains(obj) { Ok(*obj) } else { Err(FlowNodeNotFoundError) }
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
        if self.vec.is_empty() { return }
        if Direction::Stay == dir && fixed == false {
            self.fix.deactivate();
            return
        }
        // Todo: migrate linear in stockpile.
        if fixed {

        } else {

        }
    }
}

