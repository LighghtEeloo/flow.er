use dioxus::prelude::*;

fn main() {
    // launch the web app
    dioxus_web::launch(App::run);
}

struct App;

impl App {
    fn run(cx: Scope) -> Element {
        let mut count = use_state(cx, || 0);

        cx.render(rsx!(
            h1 { "High-Five counter: {count}" }
            button { onclick: move |_| count += 1, "Up high!" }
            button { onclick: move |_| count -= 1, "Down low!" }
        ))
    }
}
