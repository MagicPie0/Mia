pub const BASE_CSS: &str = r#"
    :root {
        --bg-primary: #121212;
        --bg-secondary: #1e1e1e;
        --bg-tertiary: #2d2d2d;
        --accent-primary: #bb86fc;
        --accent-secondary: #03dac6;
        --text-primary: #e1e1e1;
        --text-secondary: #a1a1a1;
        --border-radius: 8px;
        --sidebar-width: 240px;
    }
    
    body {
        background-color: var(--bg-primary);
        color: var(--text-primary);
        font-family: 'Segoe UI', Roboto, Oxygen, Ubuntu, Cantarell, sans-serif;
        margin: 0;
        padding: 0;
        min-height: 100vh;
    }
    
    .app-container {
        display: flex;
        min-height: 100vh;
    }
    
   
    
    main {
        flex: 1;
        padding: 30px;
        background-color: var(--bg-primary);
        overflow-y: auto;
    }
    
    .card {
        background-color: var(--bg-secondary);
        border-radius: var(--border-radius);
        padding: 20px;
        margin-bottom: 20px;
        box-shadow: 0 2px 8px rgba(0, 0, 0, 0.2);
    }
    
    h1, h2, h3 {
        color: var(--text-primary);
        margin-top: 0;
    }
    
    h1 {
        font-size: 2rem;
        margin-bottom: 24px;
    }
    
    .welcome-message {
        display: flex;
        flex-direction: column;
        align-items: flex-start;
        text-align: left;
        padding: 20px 0;
    }
    
    .welcome-message h1 {
        font-size: 2.5rem;
        margin-bottom: 16px;
    }
    
    .welcome-message p {
        color: var(--text-secondary);
        max-width: 600px;
        font-size: 1.1rem;
        line-height: 1.6;
        margin-bottom: 24px;
    }
    
    .accent {
        color: var(--accent-primary);
    }
    
    .btn {
        background-color: var(--accent-primary);
        color: var(--bg-primary);
        border: none;
        padding: 10px 20px;
        border-radius: var(--border-radius);
        font-weight: 600;
        cursor: pointer;type `Option<GenerationalRef<std::cell::Ref<'_, _>>>` cannot be dereferenced

can't be dereferenced
        transition: all 0.2s ease;
    }
    
    .btn:hover {
        opacity: 0.9;
        transform: translateY(-1px);
    }
"#;