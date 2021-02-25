mod architect;
mod animator;
mod dancer;
mod critic;

pub mod prelude {
    pub use super::{
        super::prelude::*,
        architect::{ Architect, FlowNode, FlowLink, FlowLinkIndex },
        animator::{ Animator },
        dancer::{ Direction, FixState, Dancer },
        critic::{ Critic, FlowNodeNotFoundError, FlowNodeExistError, FlowNodeMismatchError },
    };
    pub trait Artist<Id>: Dancer<Id> + Animator<Id> where Id: IdentityBase {}
}
