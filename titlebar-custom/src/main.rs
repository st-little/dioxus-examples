use dioxus::prelude::*;

fn main() {
    let script = r#"
    <link rel="stylesheet" href="src/assets/styles/reset.css">
    <link rel="stylesheet" href="src/assets/styles/titlebar.css">
    "#;

    dioxus::desktop::launch_cfg(app, |cfg| {
        cfg.with_custom_head(script.to_string())
            .with_window(|w| w.with_decorations(false))
    });
}

fn app(cx: Scope) -> Element {
    let window = dioxus::desktop::use_window(&cx);

    cx.render(rsx! (
        div {
            class: "titlebar",
            div {
                class: "drag-region left title",
                onmousedown: move |_| { window.drag(); },
                span {"Custom titlebar"}
            }
            div {
                class: "right",
                button {
                    class: "setting-button",
                    id: "setting",
                    onclick: |_| {
                        println!("setting!")
                    },
                    img {
                        src: "src/assets/icons/codicon_settings-gear.svg",
                        alt: "—",
                    }
                }
                button {
                    class: "titlebar-button",
                    id: "minimize",
                    onclick: |_| {
                        window.set_minimized(true);
                    },
                    img {
                        src: "src/assets/icons/codicon_chrome-minimize.svg",
                        alt: "—",
                    }
                }
                button {
                    class: "titlebar-button",
                    id: "maximize",
                    onclick: |_| {
                        window.toggle_maximized();
                    },
                    img {
                        src: "src/assets/icons/codicon_chrome-maximize.svg",
                        alt: "+",
                    }
                }
                button {
                    class: "titlebar-button",
                    id: "close",
                    onclick: |_| {
                        window.close();
                    },
                    img {
                        src: "src/assets/icons/codicon_close.svg",
                        alt: "x",
                    }
                }
            }
        }
        div { "Hello, world!" }
    ))
}
