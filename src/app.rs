use dioxus::prelude::*;
use dioxus_router::prelude::Outlet;
use crate::components::navbar::Navbar;
use crate::routes::Route;

pub fn App() -> Element {
    rsx! {
        div { class: "app-container",
            Navbar {},
            main {
                Outlet::<Route> {}
            }
        }
    }
}