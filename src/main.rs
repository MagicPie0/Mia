mod app;
mod components;
mod styles;
use app::App;

fn main() {
    dioxus::LaunchBuilder::new()
        .with_cfg(dioxus::desktop::Config::default()
            .with_menu(None)
            .with_window(dioxus::desktop::WindowBuilder::new()
                .with_maximized(true)
                .with_title("Mia")
            )
        )
        .launch(App);
}   