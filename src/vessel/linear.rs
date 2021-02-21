use crate::util::*;
use super::prelude::*;

#[derive(Clone, Debug)]
pub struct Linear<Id>
where Id: Identity
{
    vec: Vec<Id>,
}
