use clap::ArgMatches;
use flow_vessel::{Cube, CubeType, EntityField, EntityId, Filter, Identity, Symbol, TagSetField, Tube, Vessel};

use crate::{FlowerMsg, Level};


pub fn flower_arg_match(vessel: &Vessel, matches: &ArgMatches) -> FlowerMsg {
    match matches.subcommand() {
        ("entity", Some(sub_m)) => {
            entity_arg_match(vessel, sub_m)
        }
        ("node", Some(_)) => {
            FlowerMsg::Noop
        }
        ("list", Some(sub_m)) => {
            cube_arg_match(
                Cube::new(CubeType::NodeView), 
                vessel, 
                sub_m
            ) 
        }
        ("flow", Some(sub_m)) => {
            cube_arg_match(
                Cube::new(CubeType::FlowView), 
                vessel, 
                sub_m
            ) 
        }
        ("clause", Some(sub_m)) => {
            cube_arg_match(
                Cube::new(CubeType::ClauseTree), 
                vessel, 
                sub_m
            ) 
        }
        ("promised", Some(sub_m)) => {
            cube_arg_match(
                Cube::new(CubeType::PromisedLand), 
                vessel, 
                sub_m
            ) 
        }
        ("agenda", Some(sub_m)) => {
            cube_arg_match(
                Cube::new(CubeType::AgendaView), 
                vessel, 
                sub_m
            ) 
        }
        ("capture", Some(_)) => {
            FlowerMsg::Noop
        }
        ("revert", Some(_)) => {
            FlowerMsg::Noop
        }
        ("config", Some(_)) => {
            FlowerMsg::Noop
        }
        _ => FlowerMsg::Noop
    }
}

fn entity_arg_match(vessel: &Vessel, matches: &ArgMatches) -> FlowerMsg {
    let obj = matches.value_of("obj").map(|obj| {
        Identity::parse_filter(&vessel.entity_id_all(), obj)
    }).flatten();
    let mut filters = Vec::new();
    let vague_obj = if obj.is_none() {
        if let Some(fil) = matches.value_of("filter") {
            filters.push(Filter::Identity(fil.into()));
            filters.push(Filter::Face(fil.into()));
            if let Some(symbol) = Symbol::parse_vague(fil) {
                filters.push(Filter::Symbol(symbol));
            }
            filters.push(Filter::Tag(fil.into()));
        }
        vessel.entity_match_by(&filters)
    } else {
        Vec::new()
    };

    if vague_obj.len() > 1 {
        println!("Multiple matches found:");
        let mut cube = Cube::new(CubeType::NodeView);
        cube.filters = filters;
        FlowerMsg::Cube {
            cube,
            detailed: false,
            level: Level::All
        }
    } else {
        let obj = vague_obj.get(0)
            .cloned().or(obj);
        if let Some(obj) = obj {
            entity_sub_arg_match(obj, matches.subcommand())
        } else {
            println!("No match found.");
            FlowerMsg::Noop
        }
    }
}

fn entity_sub_arg_match(obj: EntityId, subcommand: (&str, Option<&ArgMatches>)) -> FlowerMsg {
    let just_cube = FlowerMsg::Cube {
        cube: Cube::new(CubeType::NodeView).with_obj(obj),
        detailed: true,
        level: Level::Unique
    };
    match subcommand {
        ("face", Some(sub_m)) => {
            if let Some(face) = sub_m.value_of("face") {
                return FlowerMsg::Tube(
                    Tube::EntityUpdate {
                        id: obj,
                        field: EntityField::Face(face.into())
                    }
                ) 
            }
        }
        ("bubble", Some(sub_m)) => {
            if let Some(bubble) = sub_m.value_of("bubble") {
                return FlowerMsg::Tube(
                    Tube::EntityUpdate {
                        id: obj,
                        field: EntityField::Bubble(bubble.into())
                    }
                ) 
            }
        }
        ("symbol", Some(sub_m)) => {
            if let Some(symbol) = sub_m.value_of("symbol") {
                if let Some(symbol) = Symbol::parse_vague(symbol) {
                    return FlowerMsg::Tube(
                        Tube::EntityUpdate {
                            id: obj,
                            field: EntityField::Symbol(symbol)
                        }
                    ) 
                }
            }
        }
        ("tag", Some(sub_m)) => {
            if let Some(tag) = sub_m.value_of("tag") {
                if sub_m.is_present("add") {
                    return FlowerMsg::Tube(
                        Tube::EntityUpdate {
                            id: obj,
                            field: EntityField::TagSet(
                                TagSetField::AddTag(tag.into())
                            )
                        }
                    ) 
                } else 
                if sub_m.is_present("del") {
                    return FlowerMsg::Tube(
                        Tube::EntityUpdate {
                            id: obj,
                            field: EntityField::TagSet(
                                TagSetField::DelTag(tag.into())
                            )
                        }
                    ) 
                } else
                if sub_m.is_present("clear") {
                    return FlowerMsg::Tube(
                        Tube::EntityUpdate {
                            id: obj,
                            field: EntityField::TagSet(
                                TagSetField::ClearTag
                            )
                        }
                    ) 
                }
            }
        }
        _ => ()
    }
    just_cube
}

fn cube_arg_match(
    mut cube: Cube, 
    vessel: &Vessel, 
    matches: &ArgMatches
) -> FlowerMsg {
    if let Some(obj) = matches.value_of("obj") {
        let obj = Identity::parse_filter(&vessel.entity_id_all(), obj);
        if let Some(obj) = obj {
            cube.set_obj(obj);
        }
    }
    if let Some(fil) = matches.value_of("filter") {
        cube.filters.push(Filter::Identity(fil.into()));
        cube.filters.push(Filter::Face(fil.into()));
        if let Some(symbol) = Symbol::parse_vague(fil) {
            cube.filters.push(Filter::Symbol(symbol));
        }
        cube.filters.push(Filter::Tag(fil.into()));
    }
    let detailed = matches.is_present("detail");
    let level = 
        if matches.is_present("recursive") {
            Level::All
        } else if matches.is_present("unique") {
            Level::Unique
        } else { 
            matches.value_of("level")
            .map_or( Level::Certain(1),
                |lv| Level::Certain( lv.parse()
                    .expect("level must be a number.")
                )
            )
        };

    // Todo..
    match cube.cube_type {
        CubeType::Inkblot => { unimplemented!() }
        CubeType::NodeView => {}
        CubeType::ClauseTree => {}
        CubeType::PromisedLand => {}
        CubeType::FlowView => {}
        CubeType::AgendaView => {}
        CubeType::TimeView => {}
        CubeType::SettingView => {}
        CubeType::Blank => { unimplemented!() }
    }

    FlowerMsg::Cube{
        cube,
        detailed,
        level
    }
}
