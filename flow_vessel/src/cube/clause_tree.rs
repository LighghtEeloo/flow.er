use std::collections::{HashMap, HashSet};
use crate::{EntityId, EntityNode, Vessel};
use super::{Cube, CubeType, CubeMember};

pub struct ClauseTreeCube {
    pub obj: EntityId,
    pub current: Option<EntityId>,
}

impl Into<Cube> for ClauseTreeCube {
    fn into(self) -> Cube {
        Cube {
            cube_type: CubeType::ClauseTree,
            obj: Some(self.obj),
            current: self.current,
            ..Cube::default()
        }
    }
}

impl From<Cube> for ClauseTreeCube {
    fn from(cube: Cube) -> Self {
        Self {
            obj: cube.obj.unwrap_or_default(),
            current: cube.current,
        }
    }
}

impl CubeMember for ClauseTreeCube {
    fn member_traverse(&self, vessel: &Vessel) -> HashSet<EntityId> {
        vessel.entity_ownership(&self.obj)
    }
}

#[derive(Default, Clone)]
pub struct ClauseTreeCore {
    pub head: EntityId,
    pub current: Option<EntityId>,
    pub node_map: HashMap<EntityId, EntityNode>
}

impl ClauseTreeCore {
    pub fn new(cube: Cube, vessel: &Vessel) -> Self {
        let cube = ClauseTreeCube::from(cube.clone());
        let head = cube.obj;
        let current = cube.current;
        let set = cube.member_traverse(vessel);
        let node_map = 
            set.into_iter()
            .filter_map(|x| vessel.node(&x).cloned())
            .map(|x| (x.id().clone(), x))
            .collect();
        Self {
            head,
            current,
            node_map
        }
    }
    pub fn head(&self) -> &EntityNode {
        self.node_map.get(&self.head).expect("root exists")
    }
    pub fn node(&self, id: &EntityId) -> &EntityNode {
        self.node_map.get(id).expect("child node exists")
    }
    pub fn head_view<F, Args, View>(&self, f: F, args: Args) -> View
    where F: Fn(&EntityNode, Args) -> View {
        f(self.head(), args)
    }
    pub fn node_view<F, View, Combinator, Args, FnArgs>
    (&self, f: F, combinator: Combinator, args: Args, f_args: FnArgs) -> Vec<View>
    where 
        F: Clone + Fn(&EntityNode, Args) -> View, 
        Combinator: Clone + Fn(View, Vec<View>) -> View,
        Args: Clone, 
        FnArgs: Clone + Fn(Args) -> Args 
    {
        self.node_view_impl(self.head, f, combinator, args, f_args)
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
        self.node(&id).children.iter().map(|id| {
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
    // Note: no larger than 5.
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

