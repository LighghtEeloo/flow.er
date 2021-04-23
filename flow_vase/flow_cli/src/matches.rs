use clap::ArgMatches;
use flow_vessel::{Cube, CubeType, Filter, Identity, Symbol, Vessel};

use crate::{FlowerMsg, Level};


pub fn flower_arg_match(vessel: &Vessel, matches: &ArgMatches) -> FlowerMsg {
    match matches.subcommand() {
        ("entity", Some(_)) => {
            FlowerMsg::Noop
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
        CubeType::Inkblot => {}
        CubeType::NodeView => {}
        CubeType::ClauseTree => {}
        CubeType::PromisedLand => {}
        CubeType::FlowView => {}
        CubeType::AgendaView => {}
        CubeType::TimeView => {}
        CubeType::SettingView => {}
        CubeType::Blank => {}
    }

    FlowerMsg::Cube{
        cube,
        detailed,
        level
    }
}
