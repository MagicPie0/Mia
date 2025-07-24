use super::base::BASE_CSS;

pub const EMAIL_CSS: &str = r#"
    .email-container {
        display: flex;
        flex-direction: column;
        height: 95vh;
        max-height: 95vh;
        overflow: hidden;
        background-color: var(--bg-primary);
    }

    .email-header {
        padding: 1rem;
        font-size: 1.5rem;
        font-weight: bold;
        background-color: var(--bg-tertiary);
        color: var(--text-primary);
        text-align: center;
    }

    .email-tabs {
        display: flex;
        border-bottom: 1px solid var(--bg-tertiary);
    }

    .tab-button {
        flex: 1;
        padding: 1rem;
        border: none;
        background-color: var(--bg-primary);
        color: var(--text-primary);
        cursor: pointer;
        font-size: 1rem;
    }

    .tab-button.active {
        background-color: var(--accent-primary);
        color: var(--bg-primary);
        font-weight: bold;
    }

    .email-list {
        flex: 1 1 auto;
        padding: 1rem;
        overflow-y: auto;
        display: flex;
        flex-direction: column;
        gap: 0.75rem;
        scrollbar-width: thin;
        scrollbar-color: var(--accent-primary) var(--bg-tertiary);
    }

    .email-list::-webkit-scrollbar {
        width: 6px;
    }
    .email-list::-webkit-scrollbar-track {
        background: var(--bg-tertiary);
    }
    .email-list::-webkit-scrollbar-thumb {
        background-color: var(--accent-primary);
        border-radius: 3px;
    }

    .email-item {
        padding: 1rem;
        border-radius: 0.5rem;
        background-color: var(--bg-tertiary);
        cursor: pointer;
        transition: transform 0.2s;
    }

    .email-item:hover {
        transform: translateY(-2px);
    }

    .email-meta {
        display: flex;
        justify-content: space-between;
        margin-bottom: 0.5rem;
        font-size: 0.9rem;
    }

    .email-sender {
        font-weight: bold;
    }

    .email-time {
        opacity: 0.7;
    }

    .email-subject {
        font-weight: bold;
        margin-bottom: 0.25rem;
    }

    .email-preview {
        opacity: 0.8;
        font-size: 0.9rem;
        white-space: nowrap;
        overflow: hidden;
        text-overflow: ellipsis;
    }
"#;

pub fn email_styles() -> String {
    format!("{}\n{}", BASE_CSS, EMAIL_CSS)
}