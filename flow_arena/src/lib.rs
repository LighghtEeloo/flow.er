//! Flow: the underlying trait of flow.er.
//! 
//! Let's start with some key concepts.
//! 
//! # A. Arena
//! 
//! An arena is a typical data structure which has:
//! 1. A map / vec to store the data.
//! 2. A relationship graph which only tracks after the keys / indices.
//! 
//! FlowArena implements an arena-like data structure, but it has integrated the data map and the relationship graph, since both of them require an Id to visit. 
//! 
//! # B. Flow
//! 
//! trait Flow requires all the sub-traits of Flow.
//! 
//! // Todo: Finish Flow docs.
//! 

mod flow;
mod arena;
mod variants;
mod ser_de;

pub use self::{
    flow::{
        Node,
        FlowBase, 
        FlowCheck,
        FlowMap,
        FlowLink, 
        FlowDevote, 
        FlowDock, 
        Direction, 
        FlowShift,
        FlowError,
        Flow, 
    },
    arena::{FlowNode, FlowArena},
    variants::{NodePure, FlowPure, GraphNode, GraphArena}
};

pub mod prelude {
    pub use crate::{
        flow::*,
        arena::*
    };
}
