#[allow(dead_code)]
#[allow(unused)]
mod cube;
mod model;

use model::Model;

fn main() {
    yew::start_app::<Model>();
}

pub mod prelude {
    pub use crate::*;

    pub use yew::web_sys::console::log_1 as LOG;
}
