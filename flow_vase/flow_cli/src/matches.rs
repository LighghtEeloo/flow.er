use clap::ArgMatches;
use flow_vessel::{
    Cube, CubeType, EntityField, EntityId, Identity, Symbol,
    TagSetField, Tube, Vessel,
};

use crate::{FlowerMsg, Level};

pub fn flower_sub_match(vessel: &Vessel, matches: &ArgMatches) -> FlowerMsg {
    match matches.subcommand() {
        ("entity", Some(sub_m)) => entity_arg_match(vessel, sub_m),
        ("node", Some(sub_m)) => node_arg_match(vessel, sub_m),
        ("list", Some(sub_m)) => {
            cube_arg_match(Cube::new(CubeType::NodeView), vessel, sub_m)
        }
        ("flow", Some(sub_m)) => {
            cube_arg_match(Cube::new(CubeType::FlowView), vessel, sub_m)
        }
        ("clause", Some(sub_m)) => {
            cube_arg_match(Cube::new(CubeType::ClauseTree), vessel, sub_m)
        }
        ("promised", Some(sub_m)) => {
            cube_arg_match(Cube::new(CubeType::PromisedLand), vessel, sub_m)
        }
        ("agenda", Some(sub_m)) => {
            cube_arg_match(Cube::new(CubeType::AgendaView), vessel, sub_m)
        }
        ("capture", Some(_)) => FlowerMsg::Noop,
        ("revert", Some(_)) => FlowerMsg::Noop,
        ("config", Some(_)) => FlowerMsg::Noop,
        _ => FlowerMsg::Noop,
    }
}

fn obj_arg_match(
    name: &str,
    vessel: &Vessel,
    matches: &ArgMatches,
) -> Option<EntityId> {
    matches
        .value_of(name)
        .map(|obj| Identity::parse_filter(&vessel.entity_id_all(), obj))
        .flatten()
}

fn entity_arg_match(vessel: &Vessel, matches: &ArgMatches) -> FlowerMsg {
    let obj = obj_arg_match("obj", vessel, matches);

    if let Some(obj) = obj {
        entity_sub_match(obj, matches.subcommand())
    } else {
        println!("No match found.");
        FlowerMsg::Noop
    }
}

fn entity_sub_match(
    obj: EntityId,
    subcommand: (&str, Option<&ArgMatches>),
) -> FlowerMsg {
    match subcommand {
        ("face", Some(sub_m)) => {
            if let Some(face) = sub_m.value_of("face") {
                return FlowerMsg::Tube(Tube::EntityUpdate {
                    id: obj,
                    field: EntityField::Face(face.into()),
                });
            }
        }
        ("bubble", Some(sub_m)) => {
            if let Some(bubble) = sub_m.value_of("bubble") {
                return FlowerMsg::Tube(Tube::EntityUpdate {
                    id: obj,
                    field: EntityField::Bubble(bubble.into()),
                });
            }
        }
        ("symbol", Some(sub_m)) => {
            if let Some(Some(symbol)) = sub_m
                .value_of("symbol")
                .map(|symbol| Symbol::parse_vague(symbol))
            {
                return FlowerMsg::Tube(Tube::EntityUpdate {
                    id: obj,
                    field: EntityField::Symbol(symbol),
                });
            }
        }
        ("tag", Some(sub_m)) => {
            if let Some(tag) = sub_m.value_of("tag") {
                if sub_m.is_present("add") {
                    return FlowerMsg::Tube(Tube::EntityUpdate {
                        id: obj,
                        field: EntityField::TagSet(TagSetField::AddTag(
                            tag.into(),
                        )),
                    });
                } else if sub_m.is_present("del") {
                    return FlowerMsg::Tube(Tube::EntityUpdate {
                        id: obj,
                        field: EntityField::TagSet(TagSetField::DelTag(
                            tag.into(),
                        )),
                    });
                } else if sub_m.is_present("clear") {
                    return FlowerMsg::Tube(Tube::EntityUpdate {
                        id: obj,
                        field: EntityField::TagSet(TagSetField::ClearTag),
                    });
                }
            }
        }
        _ => (),
    }
    just_cube(obj)
}

fn node_arg_match(vessel: &Vessel, matches: &ArgMatches) -> FlowerMsg {
    let obj = obj_arg_match("obj", vessel, matches);

    if let Some(obj) = obj {
        node_sub_match(obj, vessel, matches.subcommand())
    } else {
        match matches.subcommand() {
            ("grow", _) => FlowerMsg::Tube(Tube::EntityGrow),
            ("add", Some(sub_m)) => {
                let owner = obj_arg_match("owner", vessel, sub_m);
                let nth = sub_m
                    .value_of("nth")
                    .map(|s| s.parse().unwrap_or_default())
                    .unwrap_or_default();
                if let Some(owner) = owner {
                    FlowerMsg::Tube(Tube::EntityAdd {
                        owner,
                        dude: None,
                        idx: nth,
                    })
                } else {
                    println!("No match found.");
                    FlowerMsg::Noop
                }
            }
            _ => {
                println!("No match found.");
                FlowerMsg::Noop
            }
        }
    }
}

fn node_sub_match(
    obj: EntityId,
    vessel: &Vessel,
    subcommand: (&str, Option<&ArgMatches>),
) -> FlowerMsg {
    match subcommand {
        ("link", Some(sub_m)) => {
            let owner = obj_arg_match("owner", vessel, sub_m);
            let nth = sub_m
                .value_of("nth")
                .map(|s| s.parse().unwrap_or_default())
                .unwrap_or_default();
            if let Some(owner) = owner {
                FlowerMsg::Tube(Tube::EntityLink { obj, owner, nth })
            } else {
                println!("No match found.");
                FlowerMsg::Noop
            }
        }
        ("devote", Some(sub_m)) => {
            let owner = obj_arg_match("owner", vessel, sub_m);
            let nth = sub_m
                .value_of("nth")
                .map(|s| s.parse().unwrap_or_default())
                .unwrap_or_default();
            if let Some(owner) = owner {
                FlowerMsg::Tube(Tube::EntityDevote { obj, owner, nth })
            } else {
                println!("No match found.");
                FlowerMsg::Noop
            }
        }
        ("decay", _) => FlowerMsg::Tube(Tube::EntityDecay { obj }),
        ("erase", _) => FlowerMsg::Tube(Tube::EntityErase { obj }),
        ("delete", _) => FlowerMsg::Tube(Tube::EntityDelete { id: obj }),
        _ => just_cube(obj),
    }
}

fn cube_arg_match(
    mut cube: Cube,
    vessel: &Vessel,
    matches: &ArgMatches,
) -> FlowerMsg {
    if let Some(obj) = matches.value_of("obj") {
        let obj = Identity::parse_filter(&vessel.entity_id_all(), obj);
        if let Some(obj) = obj {
            cube.set_obj(obj);
        }
    }
    // if let Some(fil) = matches.value_of("filter") {
    //     cube.filters.push(Filter::Identity(fil.into()));
    //     cube.filters.push(Filter::Face(fil.into()));
    //     if let Some(symbol) = Symbol::parse_vague(fil) {
    //         cube.filters.push(Filter::Symbol(symbol));
    //     }
    //     cube.filters.push(Filter::Tag(fil.into()));
    // }
    let detailed = matches.is_present("detail");
    let level = if matches.is_present("recursive") {
        Level::All
    } else if matches.is_present("unique") {
        Level::Unique
    } else {
        matches.value_of("level").map_or(Level::Certain(1), |lv| {
            Level::Certain(lv.parse().expect("level must be a number."))
        })
    };

    // Todo..
    match cube.cube_type {
        CubeType::Inkblot => {
            unimplemented!()
        }
        CubeType::NodeView => {}
        CubeType::ClauseTree => {}
        CubeType::PromisedLand => {}
        CubeType::FlowView => {}
        CubeType::AgendaView => {}
        CubeType::TimeView => {}
        CubeType::SettingView => {}
        CubeType::Blank => {
            unimplemented!()
        }
    }

    FlowerMsg::Cube {
        cube,
        detailed,
        level,
    }
}

fn just_cube(obj: EntityId) -> FlowerMsg {
    FlowerMsg::Cube {
        cube: Cube::new(CubeType::NodeView).with_obj(obj),
        detailed: true,
        level: Level::Unique,
    }
}
