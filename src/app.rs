use dioxus::prelude::*;
use crate::components::navbar::Navbar;
use crate::components::chat::Chat;

pub fn App() -> Element {
    rsx! {
        div { class: "app-container",
            Navbar {},
            main {
                Chat {}
            }
        }
    }
}