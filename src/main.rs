#[cfg(feature="yew")]
fn main() {
    flow_yew::main()
    // flow_cli::main()
}

#[cfg(not(feature="yew"))]
fn main() {
}
