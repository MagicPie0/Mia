use dioxus::prelude::*;
use crate::styles::todo::todo_styles;

#[component]
pub fn TodoList() -> Element {
    let mut todos = use_signal(|| vec![
        TodoItem {
            id: 1,
            text: "Learn Dioxus".to_string(),
            completed: false,
            created_at: chrono::Local::now(),
        },
        TodoItem {
            id: 2,
            text: "Build a To-Do app".to_string(),
            completed: true,
            created_at: chrono::Local::now(),
        }
    ]);
    
    let mut new_todo_text = use_signal(|| String::new());
    let mut filter = use_signal(|| TodoFilter::All);

    rsx! {
        style { "{todo_styles()}" }
        div { class: "todo-container",
            div { class: "todo-header",
                "To-Do List"
            }
            
            div { class: "todo-input-container",
                input {
                    r#type: "text",
                    value: "{new_todo_text}",
                    placeholder: "Add a new task...",
                    oninput: move |e| new_todo_text.set(e.value()),
                    onkeydown: move |e| {
                        if e.key() == dioxus::events::Key::Enter && !new_todo_text().is_empty() {
                            let new_id = todos.read().iter().map(|t| t.id).max().unwrap_or(0) + 1;
                            todos.write().push(TodoItem {
                                id: new_id,
                                text: new_todo_text(),
                                completed: false,
                                created_at: chrono::Local::now(),
                            });
                            new_todo_text.set(String::new());
                        }
                    }
                }
                button {
                    class: "add-button",
                    disabled: new_todo_text().is_empty(),
                    onclick: move |_| {
                        if !new_todo_text().is_empty() {
                            let new_id = todos.read().iter().map(|t| t.id).max().unwrap_or(0) + 1;
                            todos.write().push(TodoItem {
                                id: new_id,
                                text: new_todo_text(),
                                completed: false,
                                created_at: chrono::Local::now(),
                            });
                            new_todo_text.set(String::new());
                        }
                    },
                    "Add"
                }
            }
            
            div { class: "todo-filters",
                button {
                    class: if filter() == TodoFilter::All { "filter-button active" } else { "filter-button" },
                    onclick: move |_| filter.set(TodoFilter::All),
                    "All"
                }
                button {
                    class: if filter() == TodoFilter::Active { "filter-button active" } else { "filter-button" },
                    onclick: move |_| filter.set(TodoFilter::Active),
                    "Active"
                }
                button {
                    class: if filter() == TodoFilter::Completed { "filter-button active" } else { "filter-button" },
                    onclick: move |_| filter.set(TodoFilter::Completed),
                    "Completed"
                }
            }
            
            div { class: "todo-items",
                {todos.read()
                    .iter()
                    .filter(|todo| match filter() {
                        TodoFilter::All => true,
                        TodoFilter::Active => !todo.completed,
                        TodoFilter::Completed => todo.completed,
                    })
                    .map(|todo| {
                        let todo_id = todo.id;
                        let todo_completed = todo.completed;
                        let todo_text = todo.text.clone();
                        let todo_created_at = todo.created_at;
                        
                        rsx! {
                            div { 
                                key: "{todo_id}",
                                class: if todo_completed { "todo-item completed" } else { "todo-item" },
                                div { 
                                    class: "todo-checkbox",
                                    onclick: move |_| {
                                        if let Some(t) = todos.write().iter_mut().find(|t| t.id == todo_id) {
                                            t.completed = !t.completed;
                                        }
                                    },
                                    if todo_completed {
                                        svg {
                                            xmlns: "http://www.w3.org/2000/svg",
                                            view_box: "0 0 24 24",
                                            width: "18",
                                            height: "18",
                                            fill: "none",
                                            stroke: "currentColor",
                                            stroke_width: "2",
                                            stroke_linecap: "round",
                                            stroke_linejoin: "round",
                                            polyline { points: "20 6 9 17 4 12" }
                                        }
                                    }
                                }
                                div { class: "todo-text",
                                    "{todo_text}"
                                }
                                div { 
                                    class: "todo-delete",
                                    onclick: move |_| {
                                        todos.write().retain(|t| t.id != todo_id);
                                    },
                                    svg {
                                        xmlns: "http://www.w3.org/2000/svg",
                                        view_box: "0 0 24 24",
                                        width: "18",
                                        height: "18",
                                        fill: "none",
                                        stroke: "currentColor",
                                        stroke_width: "2",
                                        stroke_linecap: "round",
                                        stroke_linejoin: "round",
                                        line { x1: "18", y1: "6", x2: "6", y2: "18" },
                                        line { x1: "6", y1: "6", x2: "18", y2: "18" }
                                    }
                                }
                                div { class: "todo-time",
                                    {todo_created_at.format("%Y-%m-%d %H:%M").to_string()}
                                }
                            }
                        }
                    })
                }
            }
            
            div { class: "todo-stats",
                "{todos.read().iter().filter(|t| !t.completed).count()} items left"
            }
        }
    }
}

#[derive(Clone)]
struct TodoItem {
    id: u32,
    text: String,
    completed: bool,
    created_at: chrono::DateTime<chrono::Local>,
}

#[derive(PartialEq, Clone)]
enum TodoFilter {
    All,
    Active,
    Completed,
}