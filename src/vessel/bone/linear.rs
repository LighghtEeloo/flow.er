use crate::util::*;
use crate::yew_util::*;
use super::prelude::*;


/// Linear
/// A Visualizable model for CubeView. Designed for todo-list-ish situations.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Linear<Id>
where Id: Identity
{
    pub title: String,
    pub vec: Vec<Id>,
    #[serde(skip)]
    refs: HashMap<Id, NodeRef>,
    pub pos: Option<Id>,
    pub fix: FixState<Id>,
}

impl<Id> Linear<Id>
where Id: Identity 
{
    pub fn from_flow(flow: &Flow<Id>, target: &Id) -> Self {
        let vec = flow.get(target, "linear build failed").descendant.clone();
        Self {
            title: String::new(),
            vec: vec.clone(),
            refs: HashMap::from_iter(vec.clone().into_iter().map(|x| (x, NodeRef::default())) ),
            pos: None,
            fix: FixState::Deactivated
        }
    }
    pub fn from_flow_boxed(flow: &Flow<Id>, target: &Id) -> Box<Self> {
        Box::new(Self::from_flow(flow, target))
    }
}


// // Artist

impl<Id> Artist<Id> for Linear<Id> where Id: Identity {}


// Animator

impl<Id> Animator<Id> for Linear<Id> 
where Id: Identity
{
    fn compute(&mut self) { 
        todo!() 
    }
    fn illustrate(&self) -> Html { 
        todo!() 
    }
}


// Dancer

impl<Id> Dancer<Id> for Linear<Id>
where Id: Identity
{
    fn check(&self, obj: &Id) -> Result<Id, Critic> {
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

