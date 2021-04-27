use clap::{App, Arg, SubCommand};

pub fn make_flow_app<'a, 'b>() -> App<'a, 'b> {
    let app = App::new("~~ flow.er ~~")
        .version("0.5.0")
        .about("\n~~ drifting in the flow ~~\n")
        // .author("<LighghtEeloo>")
        .settings(&[
        // clap::AppSettings::ArgRequiredElseHelp
    ])
        .subcommand(
            SubCommand::with_name("entity")
                .about("Operates entities")
                .alias("en")
                .alias("e")
                .args(&entity_match_arg())
                .subcommands(vec![
                    SubCommand::with_name("face")
                        .about("Update face")
                        .alias("fa")
                        .alias("f")
                        .arg(
                            Arg::with_name("face")
                                .value_name("EntityFace")
                                .index(1)
                                .help("Set the face of entity")
                                .takes_value(true),
                        ),
                    SubCommand::with_name("bubble")
                        .about("Update bubble")
                        .alias("bu")
                        .alias("b")
                        .arg(
                            Arg::with_name("bubble")
                                .value_name("EntityBubble")
                                .index(1)
                                .help("Set the bubble of entity")
                                .takes_value(true),
                        ),
                    SubCommand::with_name("symbol")
                        .about("Update symbol")
                        .alias("sy")
                        .alias("s")
                        .arg(
                            Arg::with_name("symbol")
                                .value_name("EntitySymbol")
                                .index(1)
                                .help("Set the symbol of entity")
                                .takes_value(true),
                        ),
                    SubCommand::with_name("tag")
                        .about("Update tag")
                        .alias("ta")
                        .alias("t")
                        .args(&[
                            Arg::with_name("tag")
                                .value_name("EntityTag")
                                .index(1)
                                .help("Set a tag to entity")
                                .takes_value(true),
                            Arg::with_name("add")
                                .short("a")
                                .long("add")
                                .help("Add the tag of entity"),
                            Arg::with_name("del")
                                .short("d")
                                .long("del")
                                .help("Del the tag from entity"),
                            Arg::with_name("clear")
                                .short("c")
                                .long("clear")
                                .help("Del the tag from entity"),
                        ]),
                    SubCommand::with_name("filter")
                        .about("Sift the entities by filters")
                        .alias("f")
                        .visible_alias("search")
                        .visible_alias("pattern")
                        .args(&[
                            Arg::with_name("arbitary")
                                .index(1)
                                .help("Arbitary filter options"),
                            Arg::with_name("identity")
                                .short("i")
                                .help("Identity filter option"),
                            Arg::with_name("face")
                                .short("f")
                                .help("Face filter option"),
                            Arg::with_name("symbol")
                                .short("s")
                                .help("Symbol filter option"),
                            Arg::with_name("tag")
                                .short("t")
                                .help("Tag filter option"),
                        ]),
                ]),
        )
        .subcommand(
            SubCommand::with_name("node")
                .about("Operates nodes")
                .alias("no")
                .alias("n")
                .args(&entity_match_arg())
                .subcommands(vec![
                    SubCommand::with_name("grow")
                        .about("Grow node")
                        .alias("gr")
                        .alias("g"),
                    SubCommand::with_name("link")
                        .about("Link node")
                        .alias("li")
                        .alias("l")
                        .args(&[
                            Arg::with_name("owner")
                                .value_name("EntityMatch")
                                .index(1)
                                .help("Set the owner of this link")
                                .takes_value(true),
                            Arg::with_name("nth")
                                .value_name("Index")
                                .index(2)
                                .help("Set the owner index of this link")
                                .takes_value(true),
                        ]),
                    SubCommand::with_name("devote")
                        .about("Devote node")
                        .alias("dev")
                        .alias("de")
                        .alias("d")
                        .args(&[
                            Arg::with_name("owner")
                                .value_name("EntityMatch")
                                .index(1)
                                .help("Set the owner of this devote")
                                .takes_value(true),
                            Arg::with_name("nth")
                                .value_name("Index")
                                .index(2)
                                .help("Set the owner index of this link")
                                .takes_value(true),
                        ]),
                    SubCommand::with_name("decay")
                        .about("Decay node")
                        .alias("dec")
                        .alias("dc"),
                    SubCommand::with_name("erase")
                        .about("Erase node")
                        .alias("er"),
                    SubCommand::with_name("add")
                        .about("Add node")
                        .alias("a")
                        .args(&[
                            Arg::with_name("owner")
                                .value_name("EntityMatch")
                                .index(1)
                                .help("Set the owner of this devote")
                                .takes_value(true),
                            Arg::with_name("nth")
                                .value_name("Index")
                                .index(2)
                                .help("Set the owner index of this link")
                                .takes_value(true),
                        ]),
                    SubCommand::with_name("delete")
                        .about("Delete node, directly and completely")
                        .alias("del"),
                ]),
        )
        .subcommand(
            SubCommand::with_name("list")
                .about("Briefly show the orphan entities")
                .alias("peek")
                .alias("pk")
                .alias("see")
                .alias("li")
                .alias("l")
                .args(&list_args()),
        )
        .subcommand(
            SubCommand::with_name("flow")
                .about("Show the entities in flow")
                .alias("fl")
                .args(&list_args())
                .args(&[Arg::with_name("non-node")
                    .short("n")
                    .long("non-node")
                    .help(
                        "List node relationship without graphically expanding",
                    )]),
        )
        .subcommand(
            SubCommand::with_name("clause")
                .about("Show the entities in clause tree")
                .alias("cl")
                .args(&list_args())
                .args(&[Arg::with_name("no-indent")
                    .short("n")
                    .long("no-indent")
                    .help("Flatten all indentions")]),
        )
        .subcommand(
            SubCommand::with_name("promised")
                .about("Show the entities in promised land")
                .alias("pr")
                .visible_alias("todo")
                .alias("to")
                .args(&list_args())
                .args(&[Arg::with_name("no-indent")
                    .short("n")
                    .long("no-indent")
                    .help("Flatten all indentions")]),
        )
        .subcommand(
            SubCommand::with_name("agenda")
                .about("Show the entities in agenda")
                .alias("ag")
                .visible_alias("calendar")
                .alias("ca")
                .args(&list_args())
                .args(&[Arg::with_name("time")
                    .short("t")
                    .long("time")
                    .help("Focus on a date")]),
        )
        .subcommand(
            SubCommand::with_name("capture")
                .about("Capture the exact moment of flow")
                .alias("cp")
                .visible_alias("snap")
                .alias("sn"),
        )
        .subcommand(
            SubCommand::with_name("revert")
                .about("Travel back and reverts your current flow")
                .alias("re")
                .args(&[
                    Arg::with_name("list")
                        .short("l")
                        .long("list")
                        .help("List all available captures"),
                    Arg::with_name("time")
                        .short("t")
                        .long("time")
                        .help("Revert to a time"),
                    Arg::with_name("version")
                        .short("v")
                        .long("version")
                        .help("Revert to a version"),
                ]),
        )
        // Todo: cube subcommands.
        .subcommand(
            SubCommand::with_name("config")
                .about("Capture the exact moment of flow")
                .alias("c")
                .visible_alias("setting")
                .alias("se")
                .args(&[
            // Todo: Settings.
        ]),
        );
    app
}

fn list_args<'a, 'b>() -> Vec<Arg<'a, 'b>> {
    vec![
        Arg::with_name("obj")
            .value_name("EntityMatch")
            .index(1)
            .visible_alias("exact")
            .help("The entity that you focus on")
            .takes_value(true),
        Arg::with_name("detail")
            .short("d")
            .long("detail")
            .help("Show all info"),
        Arg::with_name("recursive")
            .short("r")
            .long("recursive")
            .help("Show all entities recursively; priority 1"),
        Arg::with_name("unique")
            .short("u")
            .long("unique")
            .requires("obj")
            .help("Only show the exact obj; priority 2"),
        Arg::with_name("level")
            .short("l")
            .long("level")
            .help("Show within a given depth / 1 by default; priority 3")
            .takes_value(true)
            .value_name("LEVEL"),
    ]
}

fn entity_match_arg<'a, 'b>() -> Vec<Arg<'a, 'b>> {
    vec![Arg::with_name("obj")
        .value_name("EntityMatch")
        .index(1)
        .visible_alias("exact")
        .help("The entity that you focus on")
        .takes_value(true)]
}
