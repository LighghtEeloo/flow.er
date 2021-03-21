mod todo;

use yew::{Html, html};
use flow_vessel::Cube;
pub use super::{Vase, Msg};

pub struct CubeVM<T> {
    cube: Cube,
    visual: T
}


impl<T> CubeVM<T> {
    fn view(&self) -> Html {
        html! {
            <>
            </>
        }
    }
}
