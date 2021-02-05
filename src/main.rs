#[allow(dead_code)]
#[allow(unused)]
pub mod cube;
pub mod model;
pub mod error;


fn main() {
    use crate::prelude::*;
    yew::start_app::<Model>();
}

pub mod prelude {
    pub use crate::model::Model;
}


#[macro_export]
macro_rules! LOG {
    ($($arg:tt)*) => {{
        let res = format!($($arg)*);
        yew::web_sys::console::log_1(&res.into())
    }}
}

pub mod util {
    pub use crate::LOG;

    pub use std::mem;
    pub use std::collections::HashMap;
    pub use std::iter::FromIterator;

    pub use serde::{Deserialize, Serialize};
    pub use strum_macros::*;
}

pub mod yew_util {
    pub use yew::format::Json;
    pub use yew::web_sys::HtmlInputElement as InputElement;
    pub use yew::{html, Component, ComponentLink, Html, InputData, NodeRef, ShouldRender};
    pub use yew::{events::KeyboardEvent};
    pub use yew_services::storage::{Area, StorageService};
}
