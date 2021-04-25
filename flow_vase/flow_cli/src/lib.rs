use flow_vessel::{Vessel};
use flow_vessel::{Cube, Tube, CubeMeta, CubeId};

pub mod app;
pub mod matches;
pub mod view;

use matches::flower_sub_match;
use view::{flower_router_view, flower_view};

pub fn main() -> Result<(), &'static str> {
    // println!("Welcome to flow.er!");
    let matches = app::make_flow_app().get_matches();
    // println!("SubCommand: {:#?}", matches.subcommand());

    
    let f = Vessel::load();
    let mut vessel = 
    futures::executor::block_on(f)
    .map_err(|_| "load err")?
    ;
    // println!("===> {:#?} ===>", vessel);
    
    let flower_msg = flower_sub_match(&vessel, &matches);
    // println!("Updating with: {:#?}", flower_msg);
    let mirror = flower_vessel(&mut vessel, flower_msg);
    // println!("Mirror: {:#?}", mirror);
    let output = 
        match mirror.clone() {
            Mirror::Display { cube, .. } => {
                format!(
                    "=======\n\n{}\n\n=======",
                    flower_view(cube, CubeMeta::default(), CubeId::default(), &vessel)?
                )
            }
            _ => {
                flower_router_view(&vessel)?
            }
        };
    println!("{}.", output);
    
    // println!("<=== {:#?} <===", vessel);
    match mirror {
        Mirror::Write => {
            let f = vessel.save();
            futures::executor::block_on(f)
            .map_err(|_| "save err")
        }
        _ => 
            Ok(())
    }
}


#[derive(Debug)]
pub enum FlowerMsg {
    Tube (Tube),
    Cube {
        cube: Cube,
        detailed: bool,
        level: Level,
    },
    Noop
}

#[derive(Debug, Clone)]
pub enum Level {
    All, 
    Certain (usize),
    Unique,
    NonRecursive,
}

#[derive(Debug, Clone)]
pub enum Mirror {
    Display {
        cube: Cube,
        detailed: bool,
        level: Level
    },
    DisplayAll,
    Write,
    Stay
}

fn flower_vessel(vessel: &mut Vessel, flower_msg: FlowerMsg) -> Mirror {
    match flower_msg {
        FlowerMsg::Tube(tube) => {
            vessel.update_tube(tube);
            Mirror::Write
        }
        FlowerMsg::Cube{ cube, detailed, level } => {
            // println!("{:#?}", cube);
            // println!("{:?}", detailed);
            // println!("{:?}", level);
            Mirror::Display {
                cube, 
                detailed, 
                level
            }
        }
        FlowerMsg::Noop => {
            println!("Noop.");
            Mirror::DisplayAll
        }
    }
}

#[cfg(test)]
mod tests {
    // #[test]
    // fn main() {
    //     super::main()
    // }
}
