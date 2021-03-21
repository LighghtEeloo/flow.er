#[allow(unused_unsafe)]
pub mod vase;
pub fn main() {
    yew::start_app::<vase::Vase>();
}

pub fn log_obj<T: std::fmt::Debug>(name: &str, obj: T) {
    let log = format!("{}: {:#?}", name, obj);
    unsafe { yew::web_sys::console::log_1(&log.into()); }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
