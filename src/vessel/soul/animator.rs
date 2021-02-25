use crate::util::*;
use crate::yew_util::*;
use super::prelude::*;

pub trait Animator<Id>: Debug
where Id: Identity
{
    fn flow_update(&mut self, flow: &Flow<Id>) {
        unimplemented!()
    }
    fn compute(&mut self) {}
    fn illustrate(&self, vessel: &Vessel, link: &ComponentLink<Vase>) -> Html {
        unimplemented!()
    }
}
