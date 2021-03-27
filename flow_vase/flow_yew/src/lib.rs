#[allow(unused_unsafe)]
pub mod vase;
pub fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<vase::Vase>();
}

pub fn log_obj<T: std::fmt::Debug>(name: &str, obj: T) {
    let log = format!("{}: \n{:#?}", name, obj);
    // unsafe { yew::web_sys::console::log_1(&log.into()); }
    log::debug!("{}", log);
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
