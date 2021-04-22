use clap::{App, Arg, SubCommand};
use flow_vessel::{Vessel};

pub fn main() {
    // println!("Welcome to flow.er!");
    let app = make_app();
    let matches = app.get_matches();
    println!("{:#?}", matches.subcommand);
    matches.subcommand_matches("list");

    let f = Vessel::load();
    let vessel = futures::executor::block_on(f).map_err(|_| "load err").unwrap_or(Vessel::new());
    println!("{:?}", vessel);
}

fn make_app<'a, 'b>() -> App<'a, 'b> {
    let app = App::new("~~ flow.er ~~")
    .version("0.5.0")
    .about("\n~~ drifting in the flow ~~\n")
    // .author("<LighghtEeloo>")
    .subcommand(
        SubCommand::with_name("list")
        .about("Briefly shows the orphan entities")
        .alias("l")
        .args(&list_args())
    )
    .subcommand(
        SubCommand::with_name("flow")
        .about("Briefly shows the orphan entities")
        .alias("l")
        .args(&list_args())
        .args(&[ 
            Arg::with_name("node")
            .short("n")
            .long("node")
            .help("List node relationship without graphically expanding"),
        ])
    );
    app
}

fn list_args<'a, 'b>() -> Vec<Arg<'a, 'b>> {
    vec! [
        Arg::with_name("obj")
        .index(1)
        .help("The entity that you focus on")
        .takes_value(true),
    
        Arg::with_name("detail")
        .short("d")
        .long("detail")
        .help("Show all info"), 
        Arg::with_name("unique")
        .short("u")
        .long("unique")
        .requires("obj")
        .help("Only show the exact obj"), 
    
        Arg::with_name("recursive")
        .short("r")
        .long("recursive")
        .help("Shows all entities"), 
        Arg::with_name("level")
        .short("l")
        .long("level")
        .help("Shows within a given depth / all if not given")
        .takes_value(true)
        .value_name("LEVEL")
    ]
}


#[cfg(test)]
mod tests {
    #[test]
    fn main() {
        super::main()
    }
}
