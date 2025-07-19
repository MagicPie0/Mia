use super::base::BASE_CSS;

pub const NAVBAR_CSS: &str = r#"
 nav {
        width: var(--sidebar-width);
        background-color: var(--bg-secondary);
        display: flex;
        flex-direction: column;
        box-shadow: 2px 0 10px rgba(0, 0, 0, 0.3);
        justify-content: space-between;
    }
    
    .nav-main {
        padding: 20px 0;
    }
    
    .nav-footer {
        padding: 20px 0;
        border-top: 1px solid var(--bg-tertiary);
    }
    
    .nav-header {
        padding: 0 20px 20px;
        margin-bottom: 10px;
        border-bottom: 1px solid var(--bg-tertiary);
    }
    
    .nav-header h2 {
        margin: 0;
        color: var(--accent-primary);
    }
    
    nav a {
        color: var(--text-primary);
        text-decoration: none;
        padding: 12px 20px;
        margin: 4px 10px;
        border-radius: var(--border-radius);
        transition: all 0.2s ease;
        font-weight: 500;
        display: flex;
        align-items: center;
    }
    
    nav a:hover {
        background-color: var(--bg-tertiary);
     nav {
        width: var(--sidebar-width);
        background-color: var(--bg-secondary);
        display: flex;
        flex-direction: column;
        box-shadow: 2px 0 10px rgba(0, 0, 0, 0.3);
        justify-content: space-between;
    }
    
    .nav-main {
        padding: 20px 0;
    }
    
    .nav-footer {
        padding: 20px 0;
        border-top: 1px solid var(--bg-tertiary);
    }
    
    .nav-header {
        padding: 0 20px 20px;
        margin-bottom: 10px;
        border-bottom: 1px solid var(--bg-tertiary);
    }
    
    .nav-header h2 {
        margin: 0;
        color: var(--accent-primary);
    }
    
    nav a {
        color: var(--text-primary);
        text-decoration: none;
        padding: 12px 20px;
        margin: 4px 10px;
        border-radius: var(--border-radius);
        transition: all 0.2s ease;
        font-weight: 500;
        display: flex;
        align-items: center;
    }
    
    nav a:hover {
        background-color: var(--bg-tertiary);
        color: var(--accent-primary);
    }
    
    nav a.active {
        background-color: var(--accent-primary);
        color: var(--bg-primary);
        font-weight: 600;
    }
    
    nav a i {
        margin-right: 10px;
        width: 20px;
        text-align: center;
    }


        .nav-cards {
        margin-top: 20px;
        padding: 0 10px;
        display: flex;
        flex-direction: column;
        gap: 12px;
    }

    .nav-cards .card {
        background: linear-gradient(145deg, var(--bg-tertiary), var(--bg-secondary));
        border-radius: var(--border-radius);
        padding: 16px;
        border: 1px solid rgba(255, 255, 255, 0.05);
        box-shadow: 
            0 4px 6px rgba(0, 0, 0, 0.1),
            inset 0 1px 0 rgba(255, 255, 255, 0.02);
        transition: all 0.3s ease;
        position: relative;
        overflow: hidden;
    }

    .nav-cards .card::before {
        content: '';
        position: absolute;
        top: 0;
        left: 0;
        width: 3px;
        height: 100%;
        background: linear-gradient(to bottom, var(--accent-primary), var(--accent-secondary));
    }

    .nav-cards .card:hover {
        transform: translateY(-2px);
        box-shadow: 
            0 6px 12px rgba(0, 0, 0, 0.15),
            inset 0 1px 0 rgba(255, 255, 255, 0.03);
    }

    .nav-cards h2 {
        font-size: 0.95rem;
        margin-bottom: 8px;
        color: var(--accent-secondary);
        font-weight: 600;
    }

    .nav-cards p {
        font-size: 0.85rem;
        color: var(--text-secondary);
        line-height: 1.5;
        margin: 0;
    }    color: var(--accent-primary);
    }
    
    nav a.active {
        background-color: var(--accent-primary);
        color: var(--bg-primary);
        font-weight: 600;
    }
    
    nav a i {
        margin-right: 10px;
        width: 20px;
        text-align: center;
    }


        .nav-cards {
        margin-top: 20px;
        padding: 0 10px;
        display: flex;
        flex-direction: column;
        gap: 12px;
    }

    .nav-cards .card {
        background: linear-gradient(145deg, var(--bg-tertiary), var(--bg-secondary));
        border-radius: var(--border-radius);
        padding: 16px;
        border: 1px solid rgba(255, 255, 255, 0.05);
        box-shadow: 
            0 4px 6px rgba(0, 0, 0, 0.1),
            inset 0 1px 0 rgba(255, 255, 255, 0.02);
        transition: all 0.3s ease;
        position: relative;
        overflow: hidden;
    }

    .nav-cards .card::before {
        content: '';
        position: absolute;
        top: 0;
        left: 0;
        width: 3px;
        height: 100%;
        background: linear-gradient(to bottom, var(--accent-primary), var(--accent-secondary));
    }

    .nav-cards .card:hover {
        transform: translateY(-2px);
        box-shadow: 
            0 6px 12px rgba(0, 0, 0, 0.15),
            inset 0 1px 0 rgba(255, 255, 255, 0.03);
    }

    .nav-cards h2 {
        font-size: 0.95rem;
        margin-bottom: 8px;
        color: var(--accent-secondary);
        font-weight: 600;
    }

    .nav-cards p {
        font-size: 0.85rem;
        color: var(--text-secondary);
        line-height: 1.5;
        margin: 0;
    }"#;

pub fn navbar_styles() -> String{
    format!("{}\n{}", BASE_CSS, NAVBAR_CSS)
}