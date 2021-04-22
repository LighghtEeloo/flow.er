use flow_vessel::{Vessel};

pub mod app;
pub mod matches;

pub fn main() {
    // println!("Welcome to flow.er!");
    let matches = app::make_flow_app().get_matches();
    println!("{:#?}", matches.args);
    println!("{:#?}", matches.subcommand);
    // matches.subcommand_matches("list");

    let f = Vessel::load();
    let vessel = futures::executor::block_on(f).map_err(|_| "load err").unwrap_or(Vessel::new());
    println!("{:?}", vessel);
}

#[cfg(test)]
mod tests {
    // #[test]
    // fn main() {
    //     super::main()
    // }
}
