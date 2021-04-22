use clap::ArgMatches;
use flow_vessel::Tube;

pub enum FlowerMsg {
    Tube (Tube),
    Noop
}

pub fn flower_msg(matches: ArgMatches) -> FlowerMsg {
    FlowerMsg::Noop
}
