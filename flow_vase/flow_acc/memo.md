# Project Acc

## Arena

Arena is a general data model representing a flow model.

### FlowDock

`dock` adds all the nodes in another flow to self and mounts all orphan nodes to the designated node. Returns `Result<(), FlowError>`.

Err if:
1. Owner not found.
2. Node exists in current flow.

`undock` moves all the nodes under the designated node out of the current flow and unmounts them. Returns `Result<FlowArena, FlowError>`. 

Err if:
1. Obj not found.
2. Node linked by other nodes.

`snap` clones all the nodes under the designated node and unmounts them. Returns `Result<FlowArena, FlowError>`.

Err if:
1. Obj not found.

### FlowShift

```rust
enum Direction {
    Forward,
    Backward,
    Ascend,
    Descent,
}
```

`shuttle` returns the obj in the corresponding relative position.

`migrate` alters the node position by the corresponding relative position, within a single node.

`migrate_iter` alters the node position by the corresponding relative position, iteratively within the flow.


## Vessel

Vessel holds a collection of all the information that should be stored during sessions. 

### Glass

Stores all the session buffers with a `HashMap` of `Router`s and `Vec<Cube>`s. Ensures all the `Router`s exist with at least a default `Cube`.

Note that a `Buffer` will be used according to the setting.

### Cube

```rust
enum Profile {
    Where (Option<EntityId>),
    When (Time)
}
```

A profile describing the core info of the cube pane. 

