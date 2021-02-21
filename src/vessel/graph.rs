use crate::util::*;
use super::prelude::*;



#[derive(Clone, Debug)]
pub struct Graph<Id>
where Id: Identity
{
    root: Option<Id>
}


