use dioxus::{desktop::tao::window::Icon, prelude::*};
use std::path::Path;

fn main() {
    let path = "src/assets/icons/titlebar.png";
    let icon = load_icon(Path::new(path));

    dioxus::desktop::launch_cfg(app, |c| c.with_icon(icon))
}

fn app(cx: Scope) -> Element {
    cx.render(rsx! (
        div { "Hello, world!" }
    ))
}

fn load_icon(path: &Path) -> Icon {
    let (icon_rgba, icon_width, icon_height) = {
        let image = image::open(path)
            .expect("Failed to open icon path")
            .into_rgba8();
        let (width, height) = image.dimensions();
        let rgba = image.into_raw();
        (rgba, width, height)
    };
    Icon::from_rgba(icon_rgba, icon_width, icon_height).expect("Failed to open icon")
}
