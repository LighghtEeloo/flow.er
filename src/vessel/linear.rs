use crate::util::*;
use super::prelude::*;


/// Linear
/// A Visualizable model for CubeView. Designed for todo-list-ish situations.
#[derive(Clone, Debug)]
pub struct Linear<Id>
where Id: Identity
{
    vec: Vec<Id>,
    pub fix: FixState<Id>,
}
