use dioxus::prelude::*;
use crate::styles::password::password_manager_styles;
use chrono::Local;

#[component]
pub fn PasswordManager() -> Element {
    let mut passwords = use_signal(|| vec![
        PasswordEntry {
            name: "Email Account".to_string(),
            username: "user@example.com".to_string(),
            password: "secret123".to_string(),  // Use actual password for storage
            url: "https://mail.example.com".to_string(),
            last_modified: Local::now(),
        },
        PasswordEntry {
            name: "Bank Account".to_string(),
            username: "user123".to_string(),
            password: "superSecret!".to_string(),  // Use actual password for storage
            url: "https://bank.example.com".to_string(),
            last_modified: Local::now(),
        }
    ]);
    
    let mut show_add_form = use_signal(|| false);
    let mut new_entry = use_signal(|| PasswordEntry::default());
    let mut search_query = use_signal(|| String::new());
    let mut show_password = use_signal(|| false);
    let mut show_passwords = use_signal(|| vec![false; passwords.read().len()]);
    
    rsx! {
        style { "{password_manager_styles()}" }
        div { class: "password-manager-container",
            div { class: "pm-header",
                "Password Vault"
                div { class: "pm-controls",
                    input { 
                        class: "pm-search",
                        r#type: "text",
                        placeholder: "Search passwords...",
                        value: "{search_query}",
                        oninput: move |e| search_query.set(e.value())
                    }
                    button { 
                        class: "pm-add-button",
                        onclick: move |_| show_add_form.set(true),
                        "Add New"
                    }
                }
            }
            
            if *show_add_form.read() {
                div { class: "pm-form-overlay",
                    div { class: "pm-form-container",
                        div { class: "pm-form-header", "Add New Password" }
                        div { class: "pm-form",
                            div { class: "pm-form-group",
                                label { "Name" }
                                input { 
                                    r#type: "text",
                                    placeholder: "e.g. Email Account",
                                    value: "{new_entry.read().name}",
                                    oninput: move |e| new_entry.write().name = e.value()
                                }
                            }
                            div { class: "pm-form-group",
                                label { "Username/Email" }
                                input { 
                                    r#type: "text",
                                    placeholder: "username or email",
                                    value: "{new_entry.read().username}",
                                    oninput: move |e| new_entry.write().username = e.value()
                                }
                            }
                            div { class: "pm-form-group",
                                label { "Password" }
                                div { class: "pm-password-input",
                                    input { 
                                        r#type: if *show_password.read() { "text" } else { "password" },
                                        placeholder: "password",
                                        value: "{new_entry.read().password}",
                                        oninput: move |e| new_entry.write().password = e.value()
                                    }
                                    button {
                                        class: "pm-toggle-password",
                                        onclick: move |_| show_password.toggle(),
                                        if *show_password.read() {
                                            "Hide"
                                        } else {
                                            "Show"
                                        }
                                    }
                                }
                            }
                            div { class: "pm-form-group",
                                label { "URL" }
                                input { 
                                    r#type: "url",
                                    placeholder: "https://example.com",
                                    value: "{new_entry.read().url}",
                                    oninput: move |e| new_entry.write().url = e.value()
                                }
                            }
                            div { class: "pm-form-actions",
                                button { 
                                    class: "pm-cancel-button",
                                    onclick: move |_| {
                                        show_add_form.set(false);
                                        new_entry.set(PasswordEntry::default());
                                    },
                                    "Cancel"
                                }
                                button { 
                                    class: "pm-save-button",
                                    disabled: new_entry.read().name.is_empty() || new_entry.read().password.is_empty(),
                                    onclick: move |_| {
                                        passwords.write().push(new_entry());
                                        show_passwords.write().push(false);
                                        show_add_form.set(false);
                                        new_entry.set(PasswordEntry::default());
                                    },
                                    "Save"
                                }
                            }
                        }
                    }
                }
            }
            
            div { class: "pm-list",
                if passwords.read().is_empty() {
                    div { class: "pm-empty-state",
                        "No passwords saved yet. Click 'Add New' to get started."
                    }
                } else {
                    for (index, entry) in passwords.read().iter().enumerate().filter(|(_, e)| {
                        search_query.read().is_empty() ||
                        e.name.to_lowercase().contains(&search_query.read().to_lowercase()) ||
                        e.username.to_lowercase().contains(&search_query.read().to_lowercase()) ||
                        e.url.to_lowercase().contains(&search_query.read().to_lowercase())
                    }) {
                        div { class: "pm-entry",
                            div { class: "pm-entry-header",
                                div { class: "pm-entry-name", "{entry.name}" }
                                div { class: "pm-entry-actions",
                                    button { 
                                        class: "pm-copy-button", 
                                        title: "Copy password",
                                        onclick: move |_| {
                                            // Copy functionality would go here
                                        },
                                        "Copy" 
                                    }
                                    button { 
                                        class: "pm-edit-button", 
                                        title: "Edit",
                                        onclick: move |_| {
                                            // Edit functionality would go here
                                        },
                                        "Edit" 
                                    }
                                    button { 
                                        class: "pm-delete-button", 
                                        title: "Delete",
                                        onclick: move |_| {
                                            passwords.write().remove(index);
                                            show_passwords.write().remove(index);
                                        },
                                        "Delete" 
                                    }
                                }
                            }
                            div { class: "pm-entry-details",
                                div { class: "pm-detail",
                                    span { class: "pm-detail-label", "Username: " }
                                    span { class: "pm-detail-value", "{entry.username}" }
                                }
                                div { class: "pm-detail",
                                    span { class: "pm-detail-label", "Password: " }
                                    span { class: "pm-detail-value", 
                                        if *show_passwords.read().get(index).unwrap_or(&false) {
                                            "{entry.password}"
                                        } else {
                                            "••••••••"
                                        }
                                    }
                                    button {
                                        class: "pm-toggle-password",
                                        onclick: move |_| {
                                            if let Ok(mut show_pwds) = show_passwords.try_write() {
                                                if let Some(value) = show_pwds.get_mut(index) {
                                                    *value = !*value;
                                                }
                                            }
                                        },
                                        if *show_passwords.read().get(index).unwrap_or(&false) {
                                            "Hide"
                                        } else {
                                            "Show"
                                        }
                                    }
                                }
                                div { class: "pm-detail",
                                    span { class: "pm-detail-label", "URL: " }
                                    a { 
                                        href: "{entry.url}", 
                                        target: "_blank",
                                        rel: "noopener noreferrer",
                                        "{entry.url}"
                                    }
                                }
                                div { class: "pm-entry-footer",
                                    "Last modified: {format_timestamp(&entry.last_modified)}"
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[derive(Clone, Default)]
struct PasswordEntry {
    name: String,
    username: String,
    password: String,
    url: String,
    last_modified: chrono::DateTime<chrono::Local>,
}

fn format_timestamp(dt: &chrono::DateTime<chrono::Local>) -> String {
    dt.format("%Y-%m-%d %H:%M").to_string()
}