use super::base::BASE_CSS;

pub const PASSWORD_MANAGER_CSS: &str = r#"
    .password-manager-container {
        display: flex;
        flex-direction: column;
        height: 95vh;
        max-height: 95vh;
        overflow: hidden;
        background-color: var(--bg-primary);
        color: var(--text-primary);
    }

    .pm-header {
        display: flex;
        justify-content: space-between;
        align-items: center;
        padding: 1rem;
        background-color: var(--bg-secondary);
        font-size: 1.25rem;
        font-weight: bold;
        border-bottom: 1px solid var(--bg-tertiary);
    }

    .pm-controls {
        display: flex;
        gap: 0.75rem;
    }

    .pm-search {
        padding: 0.5rem 1rem;
        border-radius: 1.5rem;
        border: none;
        background-color: var(--bg-tertiary);
        color: var(--text-primary);
        min-width: 200px;
    }

    .pm-add-button {
        padding: 0.5rem 1rem;
        border-radius: 1.5rem;
        background-color: var(--accent-primary);
        color: var(--bg-primary);
        border: none;
        cursor: pointer;
        font-weight: bold;
    }

    .pm-list {
        flex: 1;
        padding: 1rem;
        overflow-y: auto;
        display: flex;
        flex-direction: column;
        gap: 1rem;
    }

    .pm-empty-state {
        display: flex;
        justify-content: center;
        align-items: center;
        height: 100%;
        color: var(--text-secondary);
        font-style: italic;
    }

    .pm-entry {
        background-color: var(--bg-tertiary);
        border-radius: 0.75rem;
        padding: 1rem;
        box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
    }

    .pm-entry-header {
        display: flex;
        justify-content: space-between;
        align-items: center;
        margin-bottom: 0.75rem;
    }

    .pm-entry-name {
        font-weight: bold;
        font-size: 1.1rem;
    }

    .pm-entry-actions {
        display: flex;
        gap: 0.5rem;
    }

    .pm-copy-button, .pm-edit-button, .pm-delete-button {
        padding: 0.25rem 0.5rem;
        border-radius: 0.5rem;
        border: none;
        cursor: pointer;
        font-size: 0.8rem;
    }

    .pm-copy-button {
        background-color: var(--accent-secondary);
        color: var(--bg-primary);
    }

    .pm-edit-button {
        background-color: var(--accent-tertiary);
        color: var(--bg-primary);
    }

    .pm-delete-button {
        background-color: var(--error);
        color: var(--bg-primary);
    }

    .pm-entry-details {
        display: flex;
        flex-direction: column;
        gap: 0.5rem;
    }

    .pm-detail {
        display: flex;
        gap: 0.5rem;
    }

    .pm-detail-label {
        font-weight: bold;
        color: var(--text-secondary);
    }

    .pm-detail-value {
        word-break: break-all;
    }

    .pm-entry-footer {
        margin-top: 0.75rem;
        font-size: 0.8rem;
        color: var(--text-secondary);
    }

    .pm-form-overlay {
        position: fixed;
        top: 0;
        left: 0;
        right: 0;
        bottom: 0;
        background-color: rgba(0, 0, 0, 0.5);
        display: flex;
        justify-content: center;
        align-items: center;
        z-index: 1000;
    }

    .pm-form-container {
        background-color: var(--bg-secondary);
        border-radius: 1rem;
        padding: 1.5rem;
        width: 90%;
        max-width: 500px;
        box-shadow: 0 4px 8px rgba(0, 0, 0, 0.2);
    }

    .pm-form-header {
        font-size: 1.25rem;
        font-weight: bold;
        margin-bottom: 1.5rem;
        text-align: center;
    }

    .pm-form {
        display: flex;
        flex-direction: column;
        gap: 1rem;
    }

    .pm-form-group {
        display: flex;
        flex-direction: column;
        gap: 0.5rem;
    }

    .pm-form-group label {
        font-weight: bold;
        color: var(--text-secondary);
    }

    .pm-form-group input {
        padding: 0.75rem;
        border-radius: 0.5rem;
        border: 1px solid var(--bg-tertiary);
        background-color: var(--bg-primary);
        color: var(--text-primary);
    }

    .pm-password-input {
        display: flex;
        gap: 0.5rem;
    }

    .pm-password-input input {
        flex: 1;
    }

    .pm-toggle-password {
        padding: 0 0.75rem;
        border-radius: 0.5rem;
        background-color: var(--bg-tertiary);
        border: none;
        cursor: pointer;
    }

    .pm-form-actions {
        display: flex;
        justify-content: flex-end;
        gap: 0.75rem;
        margin-top: 1rem;
    }

    .pm-cancel-button {
        padding: 0.75rem 1.5rem;
        border-radius: 0.5rem;
        background-color: var(--bg-tertiary);
        border: none;
        cursor: pointer;
    }

    .pm-save-button {
        padding: 0.75rem 1.5rem;
        border-radius: 0.5rem;
        background-color: var(--accent-primary);
        color: var(--bg-primary);
        border: none;
        cursor: pointer;
        font-weight: bold;
    }

    .pm-save-button:disabled {
        opacity: 0.5;
        cursor: not-allowed;
    }
"#;

pub fn password_manager_styles() -> String {
    format!("{}\n{}", BASE_CSS, PASSWORD_MANAGER_CSS)
}