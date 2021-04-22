use clap::ArgMatches;
use flow_vessel::{Cube, CubeType};

use crate::FlowerMsg;


pub fn flower_arg_match(matches: &ArgMatches) -> FlowerMsg {
    match matches.subcommand() {
        ("entity", Some(_)) => {
            FlowerMsg::Noop
        }
        ("node", Some(_)) => {
            FlowerMsg::Noop
        }
        ("list", Some(_)) => {
            FlowerMsg::Noop
        }
        ("flow", Some(sub_m)) => {
            cube_arg_match(Cube::new(CubeType::FlowView), sub_m) 
        }
        ("clause", Some(sub_m)) => {
            cube_arg_match(Cube::new(CubeType::ClauseTree), sub_m) 
        }
        ("promised", Some(sub_m)) => {
            cube_arg_match(Cube::new(CubeType::PromisedLand), sub_m) 
        }
        ("agenda", Some(sub_m)) => {
            cube_arg_match(Cube::new(CubeType::AgendaView), sub_m) 
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

fn cube_arg_match(cube: Cube, _matches: &ArgMatches) -> FlowerMsg {
    // if let Some(obj) = args.value_of("obj") {
    //     cube.set_obj()
    // }
    FlowerMsg::Cube(cube)
}
