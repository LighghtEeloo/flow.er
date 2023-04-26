use super::{FlowArena, FlowNode, Node};
use std::{fmt, hash::Hash};

#[cfg(feature = "serde_impl")]
use serde::ser::{Serialize, SerializeStruct, Serializer};
#[cfg(feature = "serde_impl")]
impl<Id, Entity> Serialize for FlowArena<Id, FlowNode<Id, Entity>>
where
    Id: Serialize + Hash + Eq + Clone,
    Entity: Serialize + Clone,
{
    fn serialize<S: Serializer>(
        &self,
        serializer: S,
    ) -> Result<S::Ok, S::Error> {
        let mut flow = serializer.serialize_struct("Flow", 2)?;
        let seq: Vec<&FlowNode<Id, Entity>> = self.node_map.values().collect();
        flow.serialize_field("node_map", &seq)?;
        flow.end()
    }
}

#[cfg(feature = "serde_impl")]
use serde::de::{
    self, Deserialize, Deserializer, MapAccess, SeqAccess, Visitor,
};
#[cfg(feature = "serde_impl")]
use std::marker::PhantomData;
#[cfg(feature = "serde_impl")]
impl<'de, Id, Entity> Deserialize<'de> for FlowArena<Id, FlowNode<Id, Entity>>
where
    Id: Deserialize<'de> + Clone + Hash + Eq,
    Entity: Deserialize<'de> + Clone,
{
    fn deserialize<D: Deserializer<'de>>(
        deserializer: D,
    ) -> Result<Self, D::Error> {
        enum Field {
            NodeMap,
        }
        impl<'de> Deserialize<'de> for Field {
            fn deserialize<D: Deserializer<'de>>(
                deserializer: D,
            ) -> Result<Field, D::Error> {
                struct FieldVisitor;

                impl<'de> Visitor<'de> for FieldVisitor {
                    type Value = Field;

                    fn expecting(
                        &self,
                        formatter: &mut fmt::Formatter,
                    ) -> fmt::Result {
                        formatter.write_str("`node_map`")
                    }

                    fn visit_str<E>(self, value: &str) -> Result<Field, E>
                    where
                        E: de::Error,
                    {
                        match value {
                            "node_map" => Ok(Field::NodeMap),
                            _ => Err(de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }

                deserializer.deserialize_identifier(FieldVisitor)
            }
        }

        struct FlowVisitor<Id: Hash + Eq + Clone, Entity: Clone> {
            marker: PhantomData<fn() -> FlowArena<Id, FlowNode<Id, Entity>>>,
        }

        impl<
                'de,
                Id: Deserialize<'de> + Clone + Hash + Eq,
                Entity: Deserialize<'de> + Clone,
            > Visitor<'de> for FlowVisitor<Id, Entity>
        {
            type Value = FlowArena<Id, FlowNode<Id, Entity>>;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("struct FlowArena")
            }

            fn visit_seq<V>(self, mut seq: V) -> Result<Self::Value, V::Error>
            where
                V: SeqAccess<'de>,
            {
                let node_vec: Vec<FlowNode<Id, Entity>> =
                    seq.next_element()?
                        .ok_or_else(|| de::Error::invalid_length(1, &self))?;
                let node_map = node_vec
                    .into_iter()
                    .map(|node| (node.id().clone(), node))
                    .collect();
                Ok(Self::Value { node_map })
            }

            fn visit_map<V>(self, mut map: V) -> Result<Self::Value, V::Error>
            where
                V: MapAccess<'de>,
            {
                let mut node_map = None;
                while let Some(key) = map.next_key()? {
                    match key {
                        Field::NodeMap => {
                            if node_map.is_some() {
                                return Err(de::Error::duplicate_field(
                                    "node_map",
                                ));
                            }
                            let node_vec: Vec<FlowNode<Id, Entity>> =
                                map.next_value()?;
                            node_map = Some(
                                node_vec
                                    .into_iter()
                                    .map(|node| (node.id().clone(), node))
                                    .collect(),
                            );
                        }
                    }
                }
                let node_map = node_map
                    .ok_or_else(|| de::Error::missing_field("node_map"))?;
                Ok(Self::Value { node_map })
            }
        }

        const FIELDS: &'static [&'static str] = &["node_map"];
        deserializer.deserialize_struct(
            "Flow",
            FIELDS,
            FlowVisitor {
                marker: PhantomData,
            },
        )
    }
}
