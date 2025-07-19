use super::base::BASE_CSS;

pub const CHAT_CSS: &str = r#"
    .chat-container {
        display: flex;
        flex-direction: column;
        height: 95vh;
        max-height: 95vh;
        overflow: hidden;
        background-color: var(--bg-primary);
    }

    .chat-messages {
        flex: 1 1 auto;
        padding: 1rem;
        overflow-y: auto;
        display: flex;
        flex-direction: column;
        gap: 0.75rem;
        scrollbar-width: thin;
        scrollbar-color: var(--accent-primary) var(--bg-tertiary);
        overscroll-behavior: contain;
    }

    /* Fix for WebKit browsers */
    .chat-messages::-webkit-scrollbar {
        width: 6px;
    }
    .chat-messages::-webkit-scrollbar-track {
        background: var(--bg-tertiary);
    }
    .chat-messages::-webkit-scrollbar-thumb {
        background-color: var(--accent-primary);
        border-radius: 3px;
    }

    .message {
        max-width: 80%;
        min-width: 120px;
        padding: 0.75rem 1rem;
        border-radius: 1rem;
        position: relative;
        word-wrap: break-word;
        overflow-wrap: break-word;
    }

    .message.user {
        align-self: flex-end;
        background-color: var(--accent-primary);
        color: var(--bg-primary);
        margin-left: auto;
        border-bottom-right-radius: 0.25rem;
    }

    .message.assistant {
        align-self: flex-start;
        background-color: var(--bg-tertiary);
        margin-right: auto;
        border-bottom-left-radius: 0.25rem;
    }

    .message-timestamp {
        font-size: 0.65rem;
        opacity: 0.7;
        margin-top: 0.25rem;
        text-align: right;
    }

    .chat-input {
        display: flex;
        gap: 0.75rem;
        padding: 1rem;
        position: sticky;
        bottom: -2;
    }

    .chat-input input {
        flex: 1;
        padding: 1rem 1rem;
        border: none;
        border-radius: 1.5rem;
        background-color: var(--bg-tertiary);
        color: var(--text-primary);
        max-height: 50px;
        font-size: 16px;
    }

    .send-button {
        padding: 0 1.25rem;
        border-radius: 1.5rem;
        background-color: var(--accent-primary);
        color: var(--bg-primary);
        border: none;
        cursor: pointer;
        min-width: 80px;
        max-height: 50px;
    }

    /* Prevent layout shift */
    body {
        overflow: hidden;
        margin: 0;
        padding: 0;
    }
"#;

pub fn chat_styles() -> String {
    format!("{}\n{}", BASE_CSS, CHAT_CSS)
}