pub mod vase;
pub fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<vase::Vase>();
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
