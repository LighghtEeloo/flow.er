use crate::util::*;
use super::prelude::*;



#[derive(Clone, Debug)]
pub struct Graph<Id>
where Id: Identity
{
    pub map: HashMap<Id, FlowNode<Id>>,
    pub pos: Option<Id>,
    pub fix: FixState<Id>,
}


