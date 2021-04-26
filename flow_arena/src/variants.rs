use super::{FlowArena, FlowNode, Node};
use std::fmt::{self, Debug};

pub type NodePure<Id> = FlowNode<Id, ()>;

pub type FlowPure<Id> = FlowArena<Id, FlowNode<Id, ()>>;

pub struct GraphNode<Id, Entity> {
    id: Id,
    entity: Entity,
    children: Vec<Id>,
}

impl<Id, Entity> Debug for GraphNode<Id, Entity>
where
    Id: Debug + Clone,
    Entity: Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct(format!("{:?}", self.id()).as_str())
            .field(">>", &self.children)
            .field("::", &self.entity)
            .finish()
    }
}

impl<Id, Entity> Node<Id> for GraphNode<Id, Entity>
where
    Id: Debug + Clone,
    Entity: Debug,
{
    fn id(&self) -> &Id {
        &self.id
    }

    fn parent(&self) -> Option<Id> {
        None
    }
    fn parent_set(&mut self, _: Id) {}
    fn parent_set_none(&mut self) {}

    fn children(&self) -> Vec<Id> {
        self.children.clone()
    }

    fn children_ref_mut(&mut self) -> &mut Vec<Id> {
        &mut self.children
    }
}

pub type GraphArena<Id, Entity> = FlowArena<Id, GraphNode<Id, Entity>>;
