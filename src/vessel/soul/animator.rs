use crate::util::*;
use crate::yew_util::*;
use super::prelude::*;

pub trait Animator<Id>: Debug
where Id: IdentityBase
{
    fn compute(&mut self) {}
    fn illustrate(&self, vessel: &Vessel, link: &ComponentLink<Vase>) -> Html {
        unimplemented!()
    }
}
