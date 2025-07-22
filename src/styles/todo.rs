use super::base::BASE_CSS;

pub const TODO_CSS: &str = r#"
    .todo-container {
        display: flex;
        flex-direction: column;
        height: 95vh;
        max-height: 95vh;
        overflow: hidden;
        background-color: var(--bg-primary);
        color: var(--text-primary);
    }

    .todo-header {
        padding: 1rem;
        font-size: 1.5rem;
        font-weight: bold;
        border-bottom: 1px solid var(--bg-tertiary);
        text-align: center;
    }

    .todo-input-container {
        display: flex;
        gap: 0.75rem;
        padding: 1rem;
        border-bottom: 1px solid var(--bg-tertiary);
    }

    .todo-input-container input {
        flex: 1;
        padding: 0.75rem 1rem;
        border: none;
        border-radius: 0.5rem;
        background-color: var(--bg-tertiary);
        color: var(--text-primary);
        font-size: 1rem;
    }

    .add-button {
        padding: 0 1.25rem;
        border-radius: 0.5rem;
        background-color: var(--accent-primary);
        color: var(--bg-primary);
        border: none;
        cursor: pointer;
    }

    .add-button:disabled {
        opacity: 0.5;
        cursor: not-allowed;
    }

    .todo-filters {
        display: flex;
        justify-content: center;
        gap: 0.5rem;
        padding: 0.75rem;
        border-bottom: 1px solid var(--bg-tertiary);
    }

    .filter-button {
        padding: 0.5rem 1rem;
        border: none;
        background: none;
        color: var(--text-secondary);
        cursor: pointer;
        border-radius: 0.25rem;
    }

    .filter-button.active {
        color: var(--accent-primary);
        font-weight: bold;
    }

    .todo-items {
        flex: 1;
        overflow-y: auto;
        padding: 0.5rem;
        scrollbar-width: thin;
        scrollbar-color: var(--accent-primary) var(--bg-tertiary);
    }

    .todo-items::-webkit-scrollbar {
        width: 6px;
    }
    .todo-items::-webkit-scrollbar-track {
        background: var(--bg-tertiary);
    }
    .todo-items::-webkit-scrollbar-thumb {
        background-color: var(--accent-primary);
        border-radius: 3px;
    }

    .todo-item {
        display: flex;
        align-items: center;
        gap: 0.75rem;
        padding: 0.75rem 1rem;
        margin-bottom: 0.5rem;
        border-radius: 0.5rem;
        background-color: var(--bg-secondary);
        transition: all 0.2s ease;
    }

    .todo-item.completed {
        opacity: 0.7;
    }

    .todo-checkbox {
        width: 1.25rem;
        height: 1.25rem;
        border: 2px solid var(--accent-primary);
        border-radius: 0.25rem;
        display: flex;
        align-items: center;
        justify-content: center;
        cursor: pointer;
    }

    .todo-text {
        flex: 1;
        word-break: break-word;
    }

    .todo-item.completed .todo-text {
        text-decoration: line-through;
        color: var(--text-secondary);
    }

    .todo-delete {
        opacity: 0;
        cursor: pointer;
        transition: opacity 0.2s ease;
    }

    .todo-item:hover .todo-delete {
        opacity: 1;
    }

    .todo-time {
        font-size: 0.75rem;
        color: var(--text-secondary);
    }

    .todo-stats {
        padding: 0.75rem;
        text-align: center;
        color: var(--text-secondary);
        border-top: 1px solid var(--bg-tertiary);
    }
"#;

pub fn todo_styles() -> String {
    format!("{}\n{}", BASE_CSS, TODO_CSS)
}