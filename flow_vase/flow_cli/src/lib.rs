use flow_vessel::{Vessel};
use flow_vessel::{Cube, Tube};
use matches::flower_arg_match;

pub mod app;
pub mod matches;

pub fn main() -> Result<(), &'static str> {
    // println!("Welcome to flow.er!");
    let matches = app::make_flow_app().get_matches();
    // println!("{:#?}", matches.args);
    println!("{:#?}", matches.subcommand());

    let flower_msg = flower_arg_match(&matches);
    
    let f = Vessel::load();
    let mut vessel = 
    futures::executor::block_on(f)
    .map_err(|_| "load err")?
    // .unwrap_or(Vessel::new())
    ;
    println!("{:#?}", vessel);

    let mirror = flower_vessel(&mut vessel, flower_msg);

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


pub enum FlowerMsg {
    Tube (Tube),
    Cube (Cube),
    Noop
}

pub enum Mirror {
    Write,
    Stay
}

fn flower_vessel(vessel: &mut Vessel, flower_msg: FlowerMsg) -> Mirror {
    let mut mirror = Mirror::Stay;
    match flower_msg {
        FlowerMsg::Tube(tube) => {
            vessel.update_tube(tube);
            mirror = Mirror::Write;
        }
        FlowerMsg::Cube(cube) => {
            println!("{:#?}", cube);
        }
        FlowerMsg::Noop => {
            println!("Noop.")
        }
    }
    mirror
}

#[cfg(test)]
mod tests {
    // #[test]
    // fn main() {
    //     super::main()
    // }
}
