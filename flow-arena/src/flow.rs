use std::{collections::HashMap, fmt::{self, Debug}, hash::Hash};

#[cfg(feature = "serde1")]
use serde::{Serialize, Deserialize};

#[derive(Clone, Default)]
#[cfg_attr(feature = "serde1", derive(Serialize, Deserialize))]
#[cfg_attr(debug_assertions, derive(PartialEq))]
pub struct Node<Id> {
    id: Id,
    parent: Option<Id>,
    children: Vec<Id>,
}

impl<Id> Node<Id> {
    pub fn id(&self) -> &Id {
        &self.id
    }
    pub fn from_id(id: Id) -> Self {
        Node {
            id,
            parent: None,
            children: Vec::new(),
        }
    }
}

#[cfg(debug_assertions)]
impl<Id: Debug> Debug for Node<Id> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct(format!("{:?}", self.id()).as_str())
            .field("parent", &self.parent)
            .field("children", &self.children)
            .finish()
    }
}

pub trait FlowLike {
    type Id;
    type Node;
    type NodeRef;
    /// ensures root and returns it
    fn root(&mut self) -> &mut Self::NodeRef;
    fn node(&self, obj: &Self::Id) -> Option<&Self::NodeRef>;
    /// inserts obj to node_map; err if exist
    fn grow(&mut self, obj: Self::Node) -> Result<(), ()>;
    /// link obj as a child of des at the nth place; err if nth > len or no obj / des
    fn devote(&mut self, obj: &Self::Id, des: &Self::Id, nth: usize) -> Result<(), ()>;
    fn devote_push(&mut self, obj: &Self::Id, des: &Self::Id) -> Result<(), ()>;
    /// removes from node_map and purges.
    fn decay(&mut self, obj: &Self::Id) -> Result<(), ()>;
    /// cuts all the links, but doesn't remove.
    fn purge(&mut self, obj: &Self::Id) -> Result<(), ()>;
}


// #[cfg_attr(feature = "serde1", derive(Serialize, Deserialize))]
#[derive(Clone)]
#[cfg_attr(debug_assertions, derive(PartialEq))]
#[cfg_attr(debug_assertions, derive(Debug))]
pub struct Flow<Id: Hash + Eq> {
    /// root: can be a Nil node or a dummy node, but must be in node_map.
    root: Id,
    node_map: HashMap<Id, Node<Id>>,
}

impl<Id: Clone + Hash + Eq + Default + Debug> Flow<Id> {
    pub fn new() -> Self {
        let node: Node<Id> = Node::default();
        let root = node.id().clone();
        let mut node_map = HashMap::new();
        node_map.insert(root.clone(), node);
        Flow { root, node_map }
    }
    /// panics if anything went wrong. Iff in debug state.
    #[cfg(debug_assertions)]
    fn check(&self) {
        for (id, node) in self.node_map.iter() {
            assert_eq!(id.clone(), node.id);
            if id.clone() == self.root {
                // root identical
                assert_eq!(Id::default(), self.root);
                // root has no parent
                assert_eq!(node.parent, None);
            } else {
                // nodes must have parent, except for root
                assert_ne!(node.parent, None);
            }
            // // children exist
            node.children.iter().for_each(|id| {
                assert!(self.node_map.get(id).is_some())
            });
            // parent exist
            if let Some(parent_id) = node.parent.clone() {
                let maybe = self.node_map.get(&parent_id);
                assert!(maybe.is_some());
                if let Some(node) = maybe {
                    assert!(node.children.iter().find(|x| x.clone() == id).is_some())
                }
            }
        }
    } 
}

impl<Id: Clone + Hash + Eq + Default + Debug> FlowLike for Flow<Id> {
    type Id = Id;
    type Node = Node<Id>;
    type NodeRef = Node<Id>;
    /// ensures root and returns it
    fn root(&mut self) -> &mut Node<Id> {
        if cfg!(debug_assertions) { self.check() };
        self.node_map.entry(Id::default()).or_default()
    }
    fn node(&self, obj: &Id) -> Option<&Node<Id>> {
        if cfg!(debug_assertions) { self.check() };
        self.node_map.get(obj)
    }
    fn grow(&mut self, mut obj: Node<Id>) -> Result<(), ()> {
        if cfg!(debug_assertions) { self.check() };
        obj.parent = Some(self.root.clone());
        match self.node_map.get(obj.id()) {
            Some(_) => Err(()),
            None => {
                let id = obj.id.clone();
                self.root().children.push(id.clone());
                self.node_map.insert(id, obj);
                Ok(())
            }
        }
    }
    fn devote(&mut self, obj: &Id, des: &Id, nth: usize) -> Result<(), ()> {
        if cfg!(debug_assertions) { self.check() };
        if self.node_map.contains_key(obj) {
            self.node_map.get_mut(des)
                .map(|owner| {
                    if nth <= owner.children.len() {
                        owner.children.insert(nth, obj.clone());
                        Some(())
                    } else {
                        None
                    }
                })
                .flatten()
                .map(|x| {
                    self.node_map.get_mut(obj).map(|obj| {
                        obj.parent = Some(des.clone());
                    });
                    Some(x)
                })
                .flatten()
        } else { None } .ok_or(())
    }
    fn devote_push(&mut self, obj: &Id, des: &Id) -> Result<(), ()> {
        if cfg!(debug_assertions) { self.check() };
        let nth = self.node_map.get(des).map(|owner| owner.children.len());
        nth.map(|nth| {
            self.devote(obj, des, nth)
        }).unwrap_or(Err(()))
    }
    /// removes from node_map and purges.
    fn decay(&mut self, obj: &Id) -> Result<(), ()> {
        if cfg!(debug_assertions) { self.check() };
        self.node_map.remove(obj).map(|_|
            self.purge(obj).ok()
        ).flatten().ok_or(())
    }
    /// cuts all the links, but doesn't remove.
    fn purge(&mut self, obj: &Id) -> Result<(), ()> {
        if cfg!(debug_assertions) { self.check() };
        for (_, node) in self.node_map.iter_mut() {
            let root = self.root.clone();
            node.children.retain(|x| *x != *obj);
            node.parent = node
                .parent
                .clone()
                .and_then(|x| if x == *obj { Some(root) } else { Some(x) });
        }
        Ok(())
    }
}

#[cfg(feature = "serde1")]
use serde::ser::{Serializer, SerializeStruct};
#[cfg(feature = "serde1")]
impl<Id: Serialize + Hash + Eq> Serialize for Flow<Id> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut flow = serializer.serialize_struct("Flow", 2)?;
        flow.serialize_field("root", &self.root)?;
        // let mut seq = serializer.serialize_seq(Some(self.node_map.len()))?;
        // for (_, e) in self.node_map {
        //     seq.serialize_element(e)?;
        // }
        let seq: Vec<&Node<Id>> = self.node_map.values().collect();
        flow.serialize_field("node_map", &seq)?;
        flow.end()
    }
}

#[cfg(feature = "serde1")]
use serde::de::{self, Deserializer, Visitor, SeqAccess, MapAccess};
#[cfg(feature = "serde1")]
use std::marker::PhantomData;
#[cfg(feature = "serde1")]
impl<'de, Id: Clone + Hash + Eq + Deserialize<'de>> Deserialize<'de> for Flow<Id> {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        enum Field { Root, NodeMap }
        impl<'de> Deserialize<'de> for Field {
            fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Field, D::Error> {
                struct FieldVisitor;

                impl<'de> Visitor<'de> for FieldVisitor {
                    type Value = Field;

                    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                        formatter.write_str("`root` or `node_map`")
                    }

                    fn visit_str<E>(self, value: &str) -> Result<Field, E>
                    where
                        E: de::Error,
                    {
                        match value {
                            "root" => Ok(Field::Root),
                            "node_map" => Ok(Field::NodeMap),
                            _ => Err(de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }

                deserializer.deserialize_identifier(FieldVisitor)
            }
        }

        struct FlowVisitor<Id: Hash + Eq> {
            marker: PhantomData<fn() -> Flow<Id>>
        }

        impl<'de, Id: Clone + Hash + Eq + Deserialize<'de>> Visitor<'de> for FlowVisitor<Id> {
            type Value = Flow<Id>;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("struct Flow")
            }

            fn visit_seq<V>(self, mut seq: V) -> Result<Self::Value, V::Error>
            where
                V: SeqAccess<'de>,
            {
                let root = seq.next_element()?
                    .ok_or_else(|| de::Error::invalid_length(0, &self))?;
                let node_vec: Vec<Node<Id>> = seq.next_element()?
                    .ok_or_else(|| de::Error::invalid_length(1, &self))?;
                let node_map = node_vec.into_iter().map(|node| (node.id.clone(), node)).collect();
                Ok(Self::Value { root, node_map })
            }

            fn visit_map<V>(self, mut map: V) -> Result<Self::Value, V::Error>
            where
                V: MapAccess<'de>,
            {
                let mut root = None;
                let mut node_map = None;
                while let Some(key) = map.next_key()? {
                    match key {
                        Field::Root => {
                            if root.is_some() {
                                return Err(de::Error::duplicate_field("root"));
                            }
                            root = Some(map.next_value()?);
                        }
                        Field::NodeMap => {
                            if node_map.is_some() {
                                return Err(de::Error::duplicate_field("node_map"));
                            }
                            let node_vec: Vec<Node<Id>> = map.next_value()?;
                            node_map = Some(node_vec.into_iter().map(|node| (node.id.clone(), node)).collect());
                        }
                    }
                }
                let root = root.ok_or_else(|| de::Error::missing_field("root"))?;
                let node_map = node_map.ok_or_else(|| de::Error::missing_field("node_map"))?;
                Ok(Self::Value { root, node_map })
            }
        }

        const FIELDS: &'static [&'static str] = &["root", "node_map"];
        deserializer.deserialize_struct("Flow", FIELDS, FlowVisitor { marker: PhantomData })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    type FlowEntity = Flow<EntityId>;
    type NodeEntity = Node<EntityId>;
    #[derive(Clone, Default, Hash, PartialEq, Eq)]
    #[cfg_attr(feature = "serde1", derive(Serialize, Deserialize))]
    struct EntityId {
        idx: u64,
    }
    impl From<u64> for EntityId {
        fn from(idx: u64) -> Self {
            EntityId { idx }
        }
    }
    impl fmt::Debug for EntityId {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "[[{:?}]]", self.idx)
        }
    }

    fn wrapper(name: &str, res: bool, flow: &FlowEntity, aloud: bool) {
        if aloud {
            println!("{}: {}", name, if res {"success"} else {"error"});
            println!("{:#?}", flow);
        }
    }

    fn make_flow(aloud: bool) -> FlowEntity {
        let mut flow: FlowEntity = Flow::new();
        let obj_vec: Vec<NodeEntity> = (0..21).collect::<Vec<u64>>().iter_mut().map(|x| Node::from_id(x.clone().into())).collect();
        wrapper("Grow", flow.grow(obj_vec[1].clone()).is_ok(), &flow, aloud);
        wrapper("Grow", flow.grow(obj_vec[2].clone()).is_ok(), &flow, aloud);
        wrapper("Grow", flow.grow(obj_vec[3].clone()).is_ok(), &flow, aloud);
        wrapper("Grow", flow.grow(obj_vec[4].clone()).is_ok(), &flow, aloud);
        wrapper("Devote", flow.devote(obj_vec[4].id(), obj_vec[1].id(), 0).is_ok(), &flow, aloud);
        wrapper("Purge", flow.purge(obj_vec[1].id()).is_ok(), &flow, aloud);
        wrapper("Decay", flow.decay(obj_vec[1].id()).is_ok(), &flow, aloud);
        if cfg!(debug_assertions) && aloud { println!("Checked."); flow.check() };
        flow
    }

    #[test]
    fn main_test() {
        make_flow(true);
    }

    #[test]
    fn root() {
        let mut flow: FlowEntity = Flow::new();
        assert_eq!(flow.root().clone(), Node::default());
    }

    #[test]
    fn serde() {
        let print_wrapper = |str: &String, aloud: bool| {
            if aloud {
                println!("{}",str)
            }
        };
        let id: EntityId = 1.into();
        print_wrapper(&serde_json::to_string(&id).unwrap(), false);
        let node: NodeEntity = Node::from_id(1.into());
        print_wrapper(&serde_json::to_string(&node).unwrap(), false);
        let flow = make_flow(false);
        // println!("{:#?}", flow);
        let str = serde_json::to_string(&flow).unwrap();
        print_wrapper(&str, false);
        let _flow: FlowEntity = serde_json::from_str(&str).unwrap();
        // println!("{:#?}", _flow);
        assert_eq!(flow, _flow)
    }
}
