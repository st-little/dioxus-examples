use dioxus::prelude::*;

fn main() {
    dioxus::desktop::launch_cfg(app, |c| c.with_window(|w| w.with_title("Custom title")));
}

fn app(cx: Scope) -> Element {
    cx.render(rsx! (
        div { "Hello, world!" }
    ))
}
