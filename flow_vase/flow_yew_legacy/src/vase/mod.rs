mod vase;
mod main_view;

pub mod prelude {
    pub use super::vase::{ Vase, VaseMsg, VaseMsg::* };
    pub use crate::vessel::prelude::*;
}

