use dioxus::prelude::*;
use rfd::FileDialog;

fn main() {
    dioxus::desktop::launch(app);
}

fn app(cx: Scope) -> Element {
    cx.render(rsx! (
        div { "File dialog" }
        // Pick folder button
        button {
            onclick: move |_| {
                let path = FileDialog::new()
                .set_directory("/")
                .pick_folder();

                println!("picked folder: {:?}", path);
            },
            "Pick folder"
        }
        // Pick file button
        button {
            onclick: move |_| {
                let path = FileDialog::new()
                .set_directory("/")
                .pick_file();

                println!("picked file: {:?}", path);
            },
            "Pick file"
        }
        // Pick files button
        button {
            onclick: move |_| {
                let path = FileDialog::new()
                .set_directory("/")
                .pick_files();

                println!("picked files: {:?}", path);
            },
            "Pick files"
        }
        // Add filter
        button {
            onclick: move |_| {
                let path = FileDialog::new()
                .add_filter("image", &["png", "jpg", "jpeg", "gif"])
                .set_directory("/")
                .pick_files();

                println!("picked images: {:?}", path);
            },
            "Pick images"
        }
    ))
}
