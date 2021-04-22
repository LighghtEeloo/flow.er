//! # FlowArena
//! 
//! A `HashMap` managed graph model with the concept of ownership.
//! 
//! ## Components
//! 
//! The `flow_arena` package consists of 
//! 1. a flow data model representation `trait Flow` and `struct FlowArena`
//! 2. a node representation `trait Node` and `struct FlowNode`
//! 3. variants like `GraphNode` and `GraphArena`.
//! 
//! ## Motivation
//! 
//! `FlowArena` grants you the ability to:
//! 1. represent a tree / graph data model in a memory-safe way
//! 2. use the concept of "ownership" to switch between the tree form and the graph form, along with:
//!    1. the ability to **move / delete nodes recursively in a controlled way** - if node A is *owned* by node B, then B's removal will trigger A's removal
//!    2. the ability to visit and traverse the data model in two different ways, namely the tree-ish way and the graph-ish way
//! 
//! In fact, it all started with Rust's ownership rules...
//! 
//! ### Arena
//! 
//! When it comes to relation driven data models, it's easy to make Rustaceans headache - just try and write a safe doubly-linked list. The concept of `Arena`, therefore, is raised to implement a simple and memory-safe representation of such models. [^1]
//! 
//! Consider a tree model. Basically it's a group of nodes where each node can point to some other nodes. How do we denote this relation? Well, we give every node a unique mark which claims its reference. We used to use pointers to cleverly mark this *uniqueness* with the help of memory addresses; but due to Rust's ownership rule, this approach is verbose to implement. So, why not mark this uniqueness by ourselves, with indices or ids? 
//! 
//! The idea of the `Arena` is simple: use a `Vec<Node>` or a `HashMap<Id, Node<Id>>` to contain all the nodes, where each node contains a collection of indices / ids, e.g. `Vec<Id>`, as well as its own data. We can mark a node with its index / id, thus we can designate a root node; to traverse, just recursively get a node and traverse its "node marker collection".
//! 
//! ### Flow
//! 
//! `FlowArena` represents a directed graph with the idea of `Arena`. More than that, it's also able to represent the "flow model".
//! 
//! Flow model is different from graph or tree model alone because it provides both ownership (tree-ish) and linkage (graph-ish) features.
//! Suppose you are removing a node from a tree, the children of it will automatically be removed, which is an implication of the "ownership" of a node to its children. In contrast, a graph node's removal will not cause the surrounding nodes to be removed.
//! 
//! Using the concept of ownership in the current graph-ish model, we yield a new kind of data model that can freely switch its behaviour between the two modes. We call such a data model a "flow model", which clearly indicates the functionality of `FlowArena`.
//! 
//! **Important**: there are some extra concepts in `FlowArena` compared to `Arena`:
//! 
//! 1. nodes have not only children(`Vec<Id>`), but also a optional parent (`Option<Id>`)
//!    1. from tree-ish prospective, parent implies ownership, `None` means root; and children is only extra notation for possible ownership
//!    2. from graph-ish prospective, children implies one-way edge
//!    3. note that a "pure link" means linkage without ownership
//!    4. however, **"ownership" implies "linkage"**: if A has parent B, then B must have A as one of its children
//! 2. any node can have its sub tree or sub graph, given that no external linkage exist 
//!    1. all the nodes owned recursively by the node are called "sub-nodes" of the node
//!    2. the node is called "tree node" or "graph node", respectively
//! 3. the nodes with no parent are called `orphan`s which is similar to a group of "root" nodes.
//! 
//! 
//! ## Usage
//! 
//! 1. directly use `struct FlowNode<Id>` and the `struct FlowArena<Id, Entity>` to represent a flow model
//! 2. impl `trait Node<Id>` and `trait Flow` - see [Trait Implementation](#triat-implementation) for reference
//! 3. when called upon, `use` the corresponding trait
//! 
//! ## Triat Implementation
//! 
//! 1. `FlowBase`: provides basic node-reflection abiliy; no check
//! 2. `FlowCheck`: checks the Flow's properties and see whether they hold
//! 3. `FlowMap`: provides hashmap functionality
//! 4. `FlowLink`: provides ability to link nodes; graph-ish
//! 5. `FlowDevote`: provides ability to devote / own nodes; tree-ish
//! 6. `FlowDock`: provides ability to cut (undock) and copy (snap) a flow from a node and paste it to another node (dock)
//! 7. `FlowShift`: provides ability to move around in flow with `Direction`
//! 8. `Flow`: checks all the traits are implemented
//! 
//! 
//! ## Related App
//! 
//! The `FlowArena` is serving as the underlying data model of [flow.er](https://github.com/LighghtEeloo/flow.er), a notebook, mindmap, todo-list and agenda app. 
//! 
//! 
//! [^1]: [no-more-tears-no-more-knots-arena-allocated-trees-in-rust](https://dev.to/deciduously/no-more-tears-no-more-knots-arena-allocated-trees-in-rust-44k6)
//! 
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
