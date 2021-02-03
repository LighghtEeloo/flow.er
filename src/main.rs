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
    pub use std::mem;

    // pub use yew::web_sys::console::log_1 as LOG;

    #[macro_export]
    macro_rules! LOG {
        ($($arg:tt)*) => {{
            let res = format!($($arg)*);
            yew::web_sys::console::log_1(&res.into())
        }}
    }
}
