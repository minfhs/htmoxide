use htmoxide::prelude::*;
use maud::PreEscaped;

pub fn head(title: &str) -> Markup {
    html! {
        head {
            meta charset="utf-8";
            meta name="viewport" content="width=device-width, initial-scale=1";
            title { (title) }
            link rel="stylesheet" href="https://cdn.jsdelivr.net/npm/@picocss/pico@2/css/pico.min.css";
            script src="https://unpkg.com/htmx.org@1.9.10" {}
            script {
                (PreEscaped(r#"
                // Clear cookies client-side when parameters are empty
                document.addEventListener('DOMContentLoaded', function() {
                    document.body.addEventListener('htmx:configRequest', function(evt) {
                        // Check all parameters and clear cookies for empty ones
                        for (const [key, value] of Object.entries(evt.detail.parameters)) {
                            if (value === '') {
                                // Delete the cookie for this parameter
                                document.cookie = key + '=; path=/; max-age=0';
                            }
                        }
                    });
                });
                "#))
            }
            (custom_styles())
        }
    }
}

pub fn header(username: Option<&str>) -> Markup {
    html! {
        header.container {
            hgroup {
                h1 { "htmoxide Demo" }
                p { "A Rust framework for building htmx applications" }
            }
            @if let Some(name) = username {
                p { 
                    "Logged in as: " strong { (name) } " | " 
                    a href="/logout" role="button" class="secondary outline" { "Logout" } 
                }
            } @else {
                p { "Visiting as guest. " a href="/login" role="button" class="secondary outline" { "Login" } }
            }
        }
    }
}

pub fn navbar(current_page: &str) -> Markup {
    html! {
        nav.container {
            ul {
                li { 
                    a href="/" class=(if current_page == "home" { "contrast" } else { "" }) {
                        "Home"
                    }
                }
                li { 
                    a href="/simple" class=(if current_page == "simple" { "contrast" } else { "" }) {
                        "Simple Demo"
                    }
                }
                li { 
                    a href="/users" class=(if current_page == "users" { "contrast" } else { "" }) {
                        "User Table"
                    }
                }
                li { 
                    a href="/combined" class=(if current_page == "combined" { "contrast" } else { "" }) {
                        "Combined View"
                    }
                }
            }
        }
    }
}

pub fn custom_styles() -> Markup {
    html! {
        style {
            r#"
            /* Minor tweaks for htmoxide components */
            .components {
                margin-top: 2rem;
            }
            #counter, #greeter, #user-table {
                margin-bottom: 2rem;
            }
            .sort-button {
                background: none;
                border: none;
                padding: 0;
                font-weight: 600;
                cursor: pointer;
                color: var(--pico-primary);
            }
            .sort-button:hover {
                text-decoration: underline;
            }
            .error {
                padding: 1rem;
                border-radius: var(--pico-border-radius);
                background-color: var(--pico-del-background-color);
                color: var(--pico-del-color);
            }
            nav ul {
                display: flex;
                gap: 1rem;
            }
            header.container p a[role="button"] {
                display: inline-block;
                padding: 0.25rem 0.75rem;
                font-size: 0.875rem;
                margin-left: 0.5rem;
            }
            
            /* Loading indicator */
            .htmx-indicator {
                display: none;
                opacity: 0;
                transition: opacity 200ms ease-in;
            }
            .htmx-request .htmx-indicator,
            .htmx-request.htmx-indicator {
                display: inline;
                opacity: 1;
            }
            
            /* Empty state styling */
            .empty-state {
                text-align: center;
                padding: 3rem 1rem;
                color: var(--muted-color);
            }
            
            /* Add hover effects for better UX */
            button:not(:disabled) {
                transition: transform 100ms ease-out;
            }
            button:not(:disabled):active {
                transform: scale(0.97);
            }
            
            /* Subtle gradient animation on swap */
            @keyframes gradient-pulse {
                0% {
                    background-position: 0% 50%;
                }
                50% {
                    background-position: 100% 50%;
                }
                100% {
                    background-position: 0% 50%;
                }
            }
            
            .htmx-swapping {
                background: linear-gradient(
                    90deg, 
                    var(--pico-background-color) 0%, 
                    var(--pico-card-background-color) 50%, 
                    var(--pico-background-color) 100%
                );
                background-size: 200% 100%;
                animation: gradient-pulse 0.6s ease-in-out;
            }
            "#
        }
    }
}
