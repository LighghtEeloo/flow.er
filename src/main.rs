// #[cfg(feature="yew")]
// fn main() {
//     flow_yew::main()
// }

#[cfg(feature = "acc")]
fn main() {
    flow_acc::main()
}

#[cfg(feature = "cli")]
fn main() {
    flow_cli::main().ok();
}

#[cfg(all(not(feature = "yew"), not(feature = "cli"), not(feature = "acc")))]
fn main() {}
