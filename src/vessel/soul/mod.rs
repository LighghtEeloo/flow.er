mod architect;
mod animator;
mod dancer;
mod critic;

pub mod prelude {
    pub use super::{
        super::prelude::*,
        animator::{ Animator },
        architect::{ Architect, FlowNode, FlowLink, FlowLinkIndex },
        dancer::{ Direction, FixState, Dancer },
        // dancer::{ Direction, FixState, RelativePath, Dancer },
        critic::{ Critic, FlowNodeNotFoundError, FlowNodeExistError, FlowNodeMismatchError },
    };
    pub trait Artist<Id>: Dancer<Id> + Animator<Id> where Id: IdentityBase {}
}
