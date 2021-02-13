#[allow(dead_code)]
#[allow(unused)]
pub mod cube;
pub mod ui;


fn main() {
    use crate::prelude::*;
    yew::start_app::<Model>();
}

pub mod prelude {
    pub use crate::ui::Model;
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
    pub use std::fmt;
    pub use std::fmt::Debug;
    pub use std::collections::HashMap;
    pub use std::iter::FromIterator;

    pub use serde::{Deserialize, Serialize};
    pub use serde_json::from_str as from_json_str;
    pub use serde_json::to_string as to_json_string;
    pub fn from_json<'a, T: Deserialize<'a>>(s: &'a str) -> T {
        from_json_str(s).expect("json deserialization failed.")
    }
    pub fn to_json<T: Serialize>(v: &T) -> String {
        to_json_string(v).expect("json serialization failed.")
    }
}

pub mod yew_util {
    pub use yew::format::Json;
    pub use yew::web_sys::HtmlInputElement as InputElement;
    pub use yew::{html, Component, ComponentLink, Html, InputData, NodeRef, ShouldRender};
    pub use yew::{events::KeyboardEvent};
    pub use yew_services::storage::{Area, StorageService};
}
