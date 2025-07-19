use dioxus::prelude::*;
use crate::styles::navbar::navbar_styles;

#[component]
pub fn Navbar() -> Element {
    rsx! {
        style { "{navbar_styles()}" }
        nav {
            div { class: "nav-main",
                div { class: "nav-header",
                    h2 { "Mia" }
                }
                a { 
                    href: "#chat", 
                    class: "active",
                    i { "💬" }
                    "Chat with Mia" 
                }
                a { 
                    href: "#toDo",
                    i { "✅" }
                    "To-Do List" 
                }
                a { 
                    href: "#emails",
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