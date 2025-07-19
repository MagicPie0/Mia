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
            div { class: "chat-messages",
                for message in messages.read().iter() {
                    div { 
                        class: if message.is_user { "message user" } else { "message assistant" },
                        div { class: "message-content",
                            "{message.text}"
                        }
                        div { class: "message-timestamp",
                            {message.timestamp.format("%H:%M").to_string()}
                        }
                    }
                }
            }
            
            div { class: "chat-input",
                input { 
                    r#type: "text",
                    value: "{input_text}",
                    placeholder: "Type your message...",
                    oninput: move |e| input_text.set(e.value()),
                    onkeydown: move |e| {
                        if e.key() == dioxus::events::Key::Enter && !input_text().is_empty() {
                            messages.write().push(Message {
                                text: input_text(),
                                is_user: true,
                                timestamp: chrono::Local::now(),
                            });
                            
                            let ai_response = get_ai_response(&input_text());
                            messages.write().push(Message {
                                text: ai_response,
                                is_user: false,
                                timestamp: chrono::Local::now(),
                            });
                            
                            input_text.set(String::new());
                        }
                    }
                }
                button { 
                    class: "send-button",
                    onclick: move |_| {
                        if !input_text().is_empty() {
                            messages.write().push(Message {
                                text: input_text(),
                                is_user: true,
                                timestamp: chrono::Local::now(),
                            });
                            
                            let ai_response = get_ai_response(&input_text());
                            messages.write().push(Message {
                                text: ai_response,
                                is_user: false,
                                timestamp: chrono::Local::now(),
                            });
                            
                            input_text.set(String::new());
                        }
                    },
                    "Send"
                }
            }
        }
    }
}

#[derive(Clone)]
struct Message {
    text: String,
    is_user: bool,
    timestamp: chrono::DateTime<chrono::Local>,
}

fn get_ai_response(input: &str) -> String {
    //This is the placeholder for the MLM call
    format!("I received your message: '{}'. This is a simulated response.", input)
}