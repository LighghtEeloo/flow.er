# FlowArena

A `HashMap` managed graph model with the concept of ownership.

## Components

The `flow_arena` package consists of `trait Flow` and `struct FlowArena`, along with its node representation `trait Node` and `struct FlowNode`.

## Motivation

`FlowArena` grants you the ability to:
1. represent a tree / graph data model in a memory-safe way
2. use the concept of "ownership" to switch between the tree form and the graph form, along with:
   1. the ability to **move / delete nodes recursively in a controlled way** - if node A is *owned* by node B, then B's removal will trigger A's removal
   2. the ability to visit and traverse the data model in two different ways, namely the tree-ish way and the graph-ish way

In fact, it all started with Rust's ownership rules...

### Arena

When it comes to relation driven data models, it's easy to make Rustaceans headache - just try and write a safe doublely-linked list. The concept of `Arena`, therefore, is raised to implement a simple and memory-safe representation of such models. [^1]

Consider a tree model. Basically it's a group of nodes where each node can point to some other nodes. How do we denote this relation? Well, we give every node a unique mark which claims its reference. We used to use pointers to cleverly mark this *uniqueness* with the help of memory addresses; but due to Rust's ownership rule, this approach is verbose to implement. So, why not mark this uniqueness by ourselves, with indices or ids? 

The idea of the `Arena` is simple: use a `Vec<Node>` or a `HashMap<Id, Node<Id>>` to contain all the nodes, where each node contains a collection of indices / ids, e.g. `Vec<Id>`, as well as its own data. We can mark a node with its index / id, thus we can designate a root node; to traverse, just recursively get a node and traverse its "node marker collection".

### Flow

`FlowArena` represents a directed graph with the idea of `Arena`. More than that, it's also able to represent the "flow model".



<!--
Flow's relationship model is then simple to understand: 
1. Each node has an id, a parent_id and a Vec of children_id.
2. All the nodes are stored in a HashMap as (id, node).
3. Nodes can be visited via id.
4. A root node ensures that all the nodes are recrusively traceable. 

And certain properties are defined:
1. All ids presented must be valid.
2. The root has no parent, while other nodes must have.
3. A node's parent must have the node as a child.
4. A node just memorizes the most recent parent.
-->

We call such a data model a "flow model", which clearly indicates the functionality of `FlowArena`.

## Usage

1. directly use `struct FlowNode<Id>` and the `struct FlowArena<Id, Entity>` to represent a flow model
2. impl `trait Node<Id>` and `trait Flow` - see [Implementation](#implementation) for reference

## Implementation

// Todo: Introduce a series of Flow traits.

## Related App

The `FlowArena` is serving as the underlying data model of [flow.er](https://github.com/LighghtEeloo/flow.er), a notebook, mindmap, todo-list and agenda app. 




[^1]: https://dev.to/deciduously/no-more-tears-no-more-knots-arena-allocated-trees-in-rust-44k6
