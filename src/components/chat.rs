use dioxus::prelude::*;
use crate::styles::chat::chat_styles;

#[component]
pub fn Chat() -> Element {
    let mut messages = use_signal(|| vec![
        Message {
            text: "Hello! I'm Mia. How can I help you today?".to_string(),
            is_user: false,
            timestamp: chrono::Local::now(),
        }
    ]);
    let mut input_text = use_signal(|| String::new());

    rsx! {
        style { "{chat_styles()}" }
        div { class: "chat-container",
            div { class: "chat-header",
                "Mia AI Assistant"
            }
            
            div { class: "chat-messages",
                for message in messages.read().iter() {
                    div { 
                        class: if message.is_user { "message user" } else { "message assistant" },
                        div { class: "message-meta",
                            if message.is_user {
                                "You"
                            } else {
                                "Mia"
                            }
                            span { class: "message-time", 
                                {message.timestamp.format("%H:%M").to_string()}
                            }
                        }
                        div { class: "message-content",
                            "{message.text}"
                        }
                    }
                }
            }
            
            div { class: "chat-input-container",
                div { class: "chat-input",
                    input { 
                        r#type: "text",
                        value: "{input_text}",
                        placeholder: "Type your message...",
                        oninput: move |e| input_text.set(e.value()),
                        onkeydown: move |e| {
                            if e.key() == dioxus::events::Key::Enter && !input_text().is_empty() {
                                handle_send(&mut messages, &mut input_text);
                            }
                        }
                    }
                    button { 
                        class: "send-button",
                        disabled: input_text().is_empty(),
                        onclick: move |_| {
                            if !input_text().is_empty() {
                                handle_send(&mut messages, &mut input_text);
                            }
                        },
                        svg {
                            xmlns: "http://www.w3.org/2000/svg",
                            view_box: "0 0 24 24",
                            width: "24",
                            height: "24",
                            fill: "none",
                            stroke: "currentColor",
                            stroke_width: "2",
                            stroke_linecap: "round",
                            stroke_linejoin: "round",
                            path {
                                d: "M22 2L11 13M22 2l-7 20-4-9-9-4 20-7z"
                            }
                        }
                    }
                }
                div { class: "chat-hint",
                    "Press Enter to send, Shift+Enter for new line"
                }
            }
        }
    }
}

fn handle_send(
    messages: &mut Signal<Vec<Message>>,
    input_text: &mut Signal<String>
) {
    let user_message = input_text();
    messages.write().push(Message {
        text: user_message.clone(),
        is_user: true,
        timestamp: chrono::Local::now(),
    });
    
    let ai_response = get_ai_response(&user_message);
    messages.write().push(Message {
        text: ai_response,
        is_user: false,
        timestamp: chrono::Local::now(),
    });
    
    input_text.set(String::new());
}

#[derive(Clone)]
struct Message {
    text: String,
    is_user: bool,
    timestamp: chrono::DateTime<chrono::Local>,
}

fn get_ai_response(input: &str) -> String {
    format!("I received your message: '{}'. This is a simulated response.", input)
}