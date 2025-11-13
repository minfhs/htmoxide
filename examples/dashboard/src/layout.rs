use htmoxide::prelude::*;

pub fn head(title: &str) -> Markup {
    html! {
        head {
            title { (title) }
            script src="https://unpkg.com/htmx.org@1.9.10" {}
            (common_styles())
        }
    }
}

pub fn header() -> Markup {
    html! {
        div.header {
            h1 { "htmoxide Demo" }
            p { "A Rust framework for building htmx applications" }
        }
    }
}

pub fn navbar(current_page: &str) -> Markup {
    html! {
        nav {
            a href="/simple" class=(if current_page == "simple" { "active" } else { "" }) {
                "Simple Demo"
            }
            " | "
            a href="/users" class=(if current_page == "users" { "active" } else { "" }) {
                "User Table"
            }
        }
    }
}

pub fn common_styles() -> Markup {
    html! {
        style {
            r#"
            body {
                font-family: system-ui, -apple-system, sans-serif;
                max-width: 1000px;
                margin: 0 auto;
                padding: 2rem;
            }
            .header {
                margin-bottom: 1rem;
            }
            nav {
                margin: 1rem 0 2rem 0;
                padding: 1rem;
                background: #f5f5f5;
                border-radius: 8px;
            }
            nav a {
                text-decoration: none;
                color: #0066cc;
                padding: 0.5rem 1rem;
                border-radius: 4px;
            }
            nav a.active {
                background: #0066cc;
                color: white;
            }
            nav a:hover:not(.active) {
                background: #e0e0e0;
            }
            .components {
                display: grid;
                gap: 2rem;
                margin-top: 2rem;
            }
            #counter, #greeter, #user-table {
                border: 2px solid #ddd;
                padding: 1.5rem;
                border-radius: 8px;
            }
            button {
                margin: 0.5rem 0.5rem 0 0;
                padding: 0.5rem 1rem;
                font-size: 1rem;
                cursor: pointer;
                background: #fff;
                border: 1px solid #ddd;
                border-radius: 4px;
            }
            button:hover {
                background: #f5f5f5;
            }
            input[type="text"] {
                padding: 0.5rem;
                font-size: 1rem;
                margin-right: 0.5rem;
                border: 1px solid #ddd;
                border-radius: 4px;
            }
            table {
                width: 100%;
                border-collapse: collapse;
                margin-top: 1rem;
            }
            th, td {
                padding: 0.75rem;
                text-align: left;
                border-bottom: 1px solid #ddd;
            }
            th {
                background: #f5f5f5;
                font-weight: 600;
            }
            .sort-button {
                margin: 0;
                padding: 0;
                background: none;
                border: none;
                font-weight: 600;
                cursor: pointer;
            }
            .sort-button:hover {
                background: #e0e0e0;
            }
            "#
        }
    }
}
