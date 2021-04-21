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
    Descend,
}
```

`shuttle` returns the obj in the corresponding relative position.

`migrate` alters the node position by the corresponding relative position, within a single node.

`migrate_iter` alters the node position by the corresponding relative position, iteratively within the flow.

// Todo: Test Arena.

## Vessel

Vessel holds a collection of all the information that should be stored during sessions. 

### Glass

```rust
struct Glass {
    router: Router,
    factory: CubeIdFactory,
    router_map: HashMap<Router, Vec<CubeId>>,
    cube_map: HashMap<CubeId, Cube>,
}
```

Stores all the session buffers with a `HashMap` of `Router`s and `Vec<CubeId>`s. Ensures all the `Router`s exist with at least a fallback valid `CubeId` corresponding to a `Cube`.

Two isomorphisms of `CubeId` are maintained by glass:
1. each space in `router_map` (a `Vec<CubeId>`) holds no same id within this vec.
2. `CubeId` can always find one and only one `Cube` in `cube_map`.

Note that the `Workspace` will be used according to the setting: if `workspace_mode` is pure, then Router::vec_router() only contains `Workspace`.

### Cube

```rust
enum Profile {
    Where (Option<EntityId>),
    When (Time),
    Why (String)
}
struct Cube {
    cube_type: CubeType,
    obj: Option<EntityId>,
    current: Option<EntityId>,
    /// obj & current are first used if requirements are already satisfied; 
    /// if more are needed, profile is then used.
    profile: Option<Profile>,
}
```

A `Cube` is a ghost of its corresponding cube pane: combined with a flow_arena, a cube can restore its pane. 

The main info is kept in obj and current; A profile records the extra info of a cube pane. 

A `Cube` can have illegal states:

```rust
let legal = match (self.cube_type, self.obj, self.current, self.profile.clone()) {
   (Inkblot,Some(_),_,None) |
   (ClauseTree,Some(_),_,None) |
   (PromisedLand,_,_,None) |
   (FlowView,Some(_),_,None) |
   (CalendarView,None,None,Some(Profile::Where(_))) |
   (CalendarView,None,None,Some(Profile::When(_))) |
   (TimeView,None,None,None) |
   (SettingView,None,None,None) |
   (Blank,_,None,Some(Profile::Why(_))) => true,
   _ => false
};
```
