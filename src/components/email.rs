use dioxus::prelude::*;
use crate::styles::email::email_styles;

#[component]
pub fn EmailClient() -> Element {
    let mut active_tab = use_signal(|| "important");
    let emails = use_signal(|| generate_emails());

    rsx! {
        style { "{email_styles()}" }
        div { class: "email-container",
            div { class: "email-header",
                "Email Client"
            }
            
            div { class: "email-tabs",
                button {
                    class: if active_tab() == "important" { "tab-button active" } else { "tab-button" },
                    onclick: move |_| active_tab.set("important"),
                    "Important"
                }
                button {
                    class: if active_tab() == "regular" { "tab-button active" } else { "tab-button" },
                    onclick: move |_| active_tab.set("regular"),
                    "Regular"
                }
                button {
                    class: if active_tab() == "spam" { "tab-button active" } else { "tab-button" },
                    onclick: move |_| active_tab.set("spam"),
                    "Spam"
                }
            }
            
            div { class: "email-list",
                for email in emails.read().iter().filter(|e| match active_tab() {
                    "important" => e.is_important,
                    "regular" => !e.is_important && !e.is_spam,
                    "spam" => e.is_spam,
                    _ => false
                }) {
                    div { 
                        class: "email-item",
                        div { class: "email-meta",
                            div { class: "email-sender", 
                                "{email.sender}"
                            }
                            div { class: "email-time", 
                                {email.timestamp.format("%H:%M").to_string()}
                            }
                        }
                        div { class: "email-subject", 
                            "{email.subject}"
                        }
                        div { class: "email-preview", 
                            "{email.preview}"
                        }
                    }
                }
            }
        }
    }
}

#[derive(Clone)]
struct Email {
    sender: String,
    subject: String,
    preview: String,
    is_important: bool,
    is_spam: bool,
    timestamp: chrono::DateTime<chrono::Local>,
}

fn generate_emails() -> Vec<Email> {
    vec![
        Email {
            sender: "John Doe".to_string(),
            subject: "Meeting Tomorrow".to_string(),
            preview: "Hi, just a reminder about our meeting tomorrow at 10 AM...".to_string(),
            is_important: true,
            is_spam: false,
            timestamp: chrono::Local::now() - chrono::Duration::hours(2),
        },
        Email {
            sender: "Amazon".to_string(),
            subject: "Your Order Has Shipped".to_string(),
            preview: "Your recent order #12345 has been shipped and will arrive...".to_string(),
            is_important: false,
            is_spam: false,
            timestamp: chrono::Local::now() - chrono::Duration::hours(5),
        },
        Email {
            sender: "Prize Committee".to_string(),
            subject: "You've Won $1,000,000!".to_string(),
            preview: "Congratulations! You've been selected as our grand prize winner...".to_string(),
            is_important: false,
            is_spam: true,
            timestamp: chrono::Local::now() - chrono::Duration::hours(10),
        },
        Email {
            sender: "Sarah Smith".to_string(),
            subject: "Project Update".to_string(),
            preview: "Here's the latest update on the project we're working on...".to_string(),
            is_important: true,
            is_spam: false,
            timestamp: chrono::Local::now() - chrono::Duration::days(1),
        },
        Email {
            sender: "Newsletter".to_string(),
            subject: "Weekly Digest".to_string(),
            preview: "Here's what's new this week in our community...".to_string(),
            is_important: false,
            is_spam: false,
            timestamp: chrono::Local::now() - chrono::Duration::days(2),
        },
    ]
}