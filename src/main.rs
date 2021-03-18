#[allow(dead_code)]
#[allow(unused)]
mod vessel;
mod vase;


fn main() {
    // use crate::prelude::*;
    // yew::start_app::<Vase>();
    flow_cli::main()
}

pub mod prelude {
    pub use super::vase::prelude::Vase;
}


#[macro_export]
macro_rules! LOG {
    ($($arg:tt)*) => {{
        let res = format!($($arg)*);
        yew::web_sys::console::log_1(&res.into())
    }}
}


#[macro_export]
macro_rules! Vasey {
    () => (
        vec![]
    );
    ( $( $x:expr ),* ) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )*
            temp_vec
        }
    };
}

// /// Globaly: Global-y, create Message from an array of GlobalMessage-s.  
// /// 
// /// "-y" for Message macros.
// #[macro_export]
// macro_rules! Globaly {
//     () => (
//         $crate::view_model::Message::_Idle
//     );
//     ( $( $x:expr ),* ) => {
//         {
//             let mut temp_vec = Vec::new();
//             $(
//                 temp_vec.push($x);
//             )*
//             $crate::view_model::Message::Global(temp_vec)
//         }
//     };
// }

pub mod universe {
    pub use crate::util::*;
    pub use crate::yew_util::*;
    pub use crate::time_util::*;
    pub use crate::vessel::prelude::*;
    pub use crate::vase::prelude::*;
}

pub mod util {
    pub use crate::{
        LOG,
    //     Cubey,
    //     Branchy,
    //     Historyly,
    //     Globaly,
        Vasey
    };

    pub use std::mem;
    pub use std::fmt;
    pub use std::fmt::Debug;
    pub use std::collections::{HashMap, HashSet};
    pub use std::iter::FromIterator;

    pub use super::json_util::*;
    pub use super::modulo::*;
    pub use super::rand_util::*;
}

mod time_util {
    pub use wasm_timer;
    pub use std::time::{ UNIX_EPOCH, SystemTime, Duration };
    pub use chrono::prelude::*;
}

mod yew_util {
    pub use yew::format::Json;
    pub use yew::web_sys::HtmlInputElement as InputElement;
    pub use yew::{ html, Bridge, Component, ComponentLink, Html, InputData, NodeRef, Properties, ShouldRender };
    pub use yew::{ events::KeyboardEvent };
    // pub use yew::agent::{ Bridged, Dispatched, Dispatcher };
    pub use yew_services::storage::{ Area, StorageService };
}


// impl

mod json_util {
    pub use serde::{Deserialize, Serialize};
    pub use serde_json::json;
    pub use serde_json::from_str as from_json_str;
    pub use serde_json::to_string as to_json_string;
    /// Basic from_json ( Json -> T ).
    pub fn from_json<'a, T: Deserialize<'a>>(s: &'a str) -> T {
        from_json_str(s).expect("json deserialization failed.")
    }
    /// Basic to_json ( T -> Json ).
    pub fn to_json<T: Serialize>(v: &T) -> String {
        to_json_string(v).expect("json serialization failed.")
    }
    /// Formatted to_json ( T -> Json ).
    pub fn export_json<T: Serialize>(v: &T) -> String {
        let obj = json!(v);
        let buf = Vec::new();
        let formatter = serde_json::ser::PrettyFormatter::with_indent(b"    ");
        let mut ser = serde_json::Serializer::with_formatter(buf, formatter);
        obj.serialize(&mut ser).unwrap();
        let res = String::from_utf8(ser.into_inner()).unwrap();
        res
    }
}

mod modulo {
    ///
    /// Modulo that handles negative numbers, works the same as Python's `%`.
    ///
    /// eg: `(a + b).modulo(c)`
    ///
    pub trait ModuloSignedExt {
        fn modulo(&self, n: Self) -> Self;
    }
    macro_rules! modulo_signed_ext_impl {
        ($($t:ty)*) => ($(
            impl ModuloSignedExt for $t {
                #[inline]
                fn modulo(&self, n: Self) -> Self {
                    (self % n + n) % n
                }
            }
        )*)
    }
    modulo_signed_ext_impl! { i8 i16 i32 i64 isize u8 u16 u32 u64 usize }
}

mod rand_util {
    pub use rand::{ thread_rng, Rng };
    pub fn random_u64() -> u64 {
        let mut rng = thread_rng();
        rng.gen()
    }
}
