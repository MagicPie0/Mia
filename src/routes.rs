use dioxus::prelude::*;
use dioxus_router::prelude::*;
use crate::components::chat::Chat;
use crate::components::todo::TodoList;
use crate::components::email::EmailClient;
use crate::components::password::PasswordManager;
use crate::App;

#[derive(Clone, Routable, Debug, PartialEq)]
#[rustfmt::skip]
pub enum Route {
    #[layout(App)]
    #[route("/")]
    Chat {},

    #[route("/todo")]
    TodoList {},

    #[route("/email")]
    EmailClient {},

    #[route("/password")]
    PasswordManager {}
}