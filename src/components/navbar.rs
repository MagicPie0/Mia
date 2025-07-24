use dioxus::prelude::*;
use crate::styles::navbar::navbar_styles;
use dioxus_router::prelude::*;
use crate::routes::Route;

#[component]
pub fn Navbar() -> Element {
    let route = use_route::<Route>();
    fn is_active(route: &Route, target: &Route) -> &'static str{
        if route == target {"active"} else {""}
    } 

    rsx! {
        style { "{navbar_styles()}" }
        nav {
            div { class: "nav-main",
                div { class: "nav-header",
                    h2 { "Mia" }
                }
                Link { 
                    to: Route::Chat {  }, 
                    class: is_active(&route, &Route::Chat {}),
                    i { "💬" }
                    "Chat with Mia" 
                }
                Link { 
                    to: Route::TodoList {  },
                    class: is_active(&route, &Route::TodoList {}),
                    i { "✅" }
                    "To-Do List" 
                }
                Link { 
                    to: Route::EmailClient {  },
                    class: is_active(&route, &Route::EmailClient {}),
                    i { "📧" }
                    "My Emails" 
                }
                a { 
                    href: "#passwords",
                    i { "🔑" }
                    "My Passwords" 
                }
                
                div { class: "nav-cards",
                    div { class: "card",
                        h2 { "Quick Stats" }
                        p { "You have 3 new emails, 5 pending tasks, and all passwords are secure." }
                    }
                    
                    div { class: "card",
                        h2 { "Recent Activity" }
                        p { "No recent activity. Start using Mia to see your history here." }
                    }
                }
            }
            div { class: "nav-footer",
                a { 
                    href: "#settings",
                    i { "⚙️" }
                    "Settings" 
                }
            }
        }
    }
}