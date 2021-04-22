use flow_arena::{FlowBase, FlowDock, FlowMap, Node};

use crate::{Cube, CubeId, CubeMeta, EntityFlow, EntityId, EntityNode, Vessel};

#[derive(Clone)]
pub struct ClauseTreeCore {
    pub cube_id: CubeId,
    pub meta: CubeMeta,
    pub head: EntityId,
    pub current: Option<EntityId>,
    pub flow: EntityFlow,
}

impl ClauseTreeCore {
    pub fn from_router_cube(vessel: &Vessel, (meta, cube_id, cube): (CubeMeta, CubeId, Cube)) -> Option<Self> {
        let head = cube.obj?;
        let current = cube.current;
        let (mut flow, vec) = vessel.flow.snap_owned(&head).ok()?;
        let mut node = vessel.node(&head).cloned()?;
        *node.children_ref_mut() = vec;
        flow.grow(node).ok()?;
        Some ( Self {
            cube_id,
            meta,
            head,
            current,
            flow,
        })
    }
    pub fn head(&self) -> &EntityNode {
        self.flow.node(&self.head).expect("head exists")
    }
    pub fn node(&self, id: &EntityId) -> &EntityNode {
        self.flow.node(id).expect("child node exists")
    }
    pub fn head_view<F, Args, View>(&self, f: F, args: Args) -> View
    where F: Fn(&EntityNode, Args) -> View {
        f(self.head(), args)
    }
    pub fn node_view<F, View, Combinator, Args, FnArgs>
    (&self, f: F, combinator: Combinator, args: Args, fn_args: FnArgs) -> Vec<View>
    where 
        F: Clone + Fn(&EntityNode, Args) -> View, 
        Combinator: Clone + Fn(View, Vec<View>) -> View,
        Args: Clone, 
        FnArgs: Clone + Fn(Args) -> Args 
    {
        self.node_view_impl(self.head, f, combinator, args, fn_args)
    }
    fn node_view_impl<F, View, Combinator, Args, FnArgs>
    (&self, id: EntityId, f: F, combinator: Combinator, args: Args, f_args: FnArgs) -> Vec<View>
    where 
        F: Clone + Fn(&EntityNode, Args) -> View, 
        Combinator: Clone + Fn(View, Vec<View>) -> View,
        Args: Clone, 
        FnArgs: Clone + Fn(Args) -> Args 
    {
        let args = f_args(args);
        self.node(&id).children().iter().map(|id| {
            combinator (
                f(self.node(id), args.clone()),
                self.node_view_impl(id.clone(), f.clone(), combinator.clone(), args.clone(), f_args.clone())
            )
        }).collect()
    }
}

/*
fn node_view(
    clause_node: ClauseNode, 
    vessel: &Vessel,
    idx: usize, 
    node: &EntityNode, 
    owner: EntityId,
    ref_map: &HashMap<EntityId, NodeRef>, 
    meta: CubeMeta, 
    indent: usize
) -> Html {
    let node_ref = ref_map.get(node.entity.id()).cloned().unwrap_or_default();
    let clause_node_view = clause_node.view(
        idx, 
        &node.entity, 
        node_ref,
        owner,
        meta,
        indent,
    );
    // no larger than 5.
    let children_view: Vec<Html> = if indent < 5 && !node.entity.blocked {
        node.children.iter().enumerate().map(|(idx, &id)| {
            let clause_node = ClauseNode {
                id,
                node_ref: ref_map.get(&id).cloned().unwrap_or_default(),
                link: clause_node.link.clone(),
            };
            let node = vessel.node(&clause_node.id).expect("must exist");
            let owner = node.parent;
            if let Some(owner) = owner {
                node_view(clause_node, vessel, idx, node, owner, ref_map, meta, indent + 1)
            } else {
                html! {}
            }
        }).collect() } else { Vec::new() };
    html! {
        <>
            { clause_node_view }
            { children_view }
        </>
    }
}
 */

