use dioxus::prelude::*;
use dioxus_free_icons::icons::fa_brands_icons::FaRust;
use dioxus_free_icons::Icon;

fn main() {
    dioxus::desktop::launch(app);
}

fn app(cx: Scope) -> Element {
    cx.render(rsx! (
        div { "Hello, world!" }
        Icon {
            width: 30,
            height: 30,
            fill: "black",
            icon: FaRust,
        }
    ))
}
