mod app;
mod components;
mod styles;
mod routes;

use app::App;
use routes::Route;

use dioxus::prelude::*;
use dioxus_router::prelude::*;

fn main() {
    dioxus::LaunchBuilder::new()
        .with_cfg(
            dioxus::desktop::Config::default()
                .with_menu(None)
                .with_window(
                    dioxus::desktop::WindowBuilder::new()
                        .with_maximized(true)
                        .with_title("Mia"),
                ),
        )
        .launch(|| {
           rsx! {
            Router::<Route> {}
           }
        });
}
