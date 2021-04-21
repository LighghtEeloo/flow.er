use super::*;

use serde::ser::{Serialize, Serializer, SerializeStruct};
impl Serialize for Glass {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut flow = 
            serializer.serialize_struct("Flow", 2)?;
        let seq: Vec<(&CubeId, &Cube)> = self.cube_map.iter().collect();
        flow.serialize_field("router", &self.router)?;
        flow.serialize_field("factory", &self.factory)?;
        flow.serialize_field("router_map", &self.router_map)?;
        flow.serialize_field("cube_map", &seq)?;
        flow.end()
    }
}

use serde::de::{self, Deserialize, Deserializer, Visitor, MapAccess};
use std::marker::PhantomData;
impl<'de> Deserialize<'de> for Glass {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        enum Field { Router, Factory, RouterMap, CubeMap }
        use std::fmt;
        impl<'de> Deserialize<'de> for Field {
            fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Field, D::Error> {
                struct FieldVisitor;

                impl<'de> Visitor<'de> for FieldVisitor {
                    type Value = Field;

                    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                        formatter.write_str("`cube_map`")
                    }

                    fn visit_str<E>(self, value: &str) -> Result<Field, E>
                    where
                        E: de::Error,
                    {
                        match value {
                            "router" => Ok(Field::Router),
                            "factory" => Ok(Field::Factory),
                            "router_map" => Ok(Field::RouterMap),
                            "cube_map" => Ok(Field::CubeMap),
                            _ => Err(de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }

                deserializer.deserialize_identifier(FieldVisitor)
            }
        }

        struct GlassVisitor {
            marker: PhantomData<fn() -> Glass>
        }

        impl<'de> Visitor<'de> for GlassVisitor {
            type Value = Glass;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("struct Glass")
            }

            // fn visit_seq<V>(self, mut seq: V) -> Result<Self::Value, V::Error>
            // where
            //     V: SeqAccess<'de>,
            // {
            //     let cube_vec: Vec<(CubeId, Cube)> = seq.next_element()?
            //         .ok_or_else(|| de::Error::invalid_length(1, &self))?;
            //     let cube_map = cube_vec.into_iter().collect();
            //     Ok(Self::Value { cube_map })
            // }

            fn visit_map<V>(self, mut map: V) -> Result<Self::Value, V::Error>
            where
                V: MapAccess<'de>,
            {
                let mut router = None;
                let mut factory = None;
                let mut router_map = None;
                let mut cube_map = None;
                while let Some(key) = map.next_key()? {
                    match key {
                        Field::Router => {
                            if router.is_some() {
                                return Err(de::Error::duplicate_field("router"));
                            }
                            router = map.next_value()?;
                        }
                        Field::Factory => {
                            if factory.is_some() {
                                return Err(de::Error::duplicate_field("factory"));
                            }
                            factory = map.next_value()?;
                        }
                        Field::RouterMap => {
                            if router_map.is_some() {
                                return Err(de::Error::duplicate_field("router_map"));
                            }
                            router_map = map.next_value()?;
                        }
                        Field::CubeMap => {
                            if cube_map.is_some() {
                                return Err(de::Error::duplicate_field("cube_map"));
                            }
                            let cube_vec: Vec<(CubeId, Cube)> = map.next_value()?;
                            cube_map = Some(cube_vec.into_iter().collect());
                        }
                    }
                }
                let router = router.unwrap_or_default();
                let factory = factory.ok_or_else(|| de::Error::missing_field("factory"))?;
                let router_map = router_map.ok_or_else(|| de::Error::missing_field("router_map"))?;
                let cube_map = cube_map.ok_or_else(|| de::Error::missing_field("cube_map"))?;
                Ok(Self::Value { router, factory, router_map, cube_map })
            }
        }

        const FIELDS: &'static [&'static str] = &["router", "factory", "router_map", "cube_map"];
        deserializer.deserialize_struct("Glass", FIELDS, GlassVisitor { marker: PhantomData })
    }
}
