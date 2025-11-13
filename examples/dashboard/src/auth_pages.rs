use htmoxide::prelude::*;
use axum::Form;
use axum::extract::Query;
use serde::Deserialize;
use crate::auth::{AuthSession, Credentials};
use crate::layout::head;

#[derive(Deserialize)]
pub struct RedirectParams {
    #[serde(default)]
    redirect: String,
}

pub async fn login_page(Query(params): Query<RedirectParams>) -> Page {
    let redirect = if params.redirect.is_empty() {
        "/".to_string()
    } else {
        params.redirect
    };
    
    html! {
        (head("Login - htmoxide"))
        body {
            main.container {
                article style="max-width: 500px; margin: 4rem auto;" {
                    hgroup {
                        h1 { "Login" }
                        p { "Demo credentials: admin/admin123 or user/user123" }
                    }
                    
                    form method="post" action=(format!("/login?redirect={}", urlencoding::encode(&redirect))) {
                        label {
                            "Username"
                            input type="text" name="username" required autocomplete="username";
                        }
                        label {
                            "Password"
                            input type="password" name="password" required autocomplete="current-password";
                        }
                        button type="submit" { "Login" }
                    }
                }
            }
        }
    }
    .into()
}

pub async fn login_handler(
    Query(params): Query<RedirectParams>,
    mut auth_session: AuthSession,
    Form(creds): Form<Credentials>,
) -> axum::response::Redirect {
    let user = auth_session.authenticate(creds.clone()).await.ok().flatten();

    if let Some(user) = user {
        let _ = auth_session.login(&user).await;
        let redirect_to = if params.redirect.is_empty() {
            "/"
        } else {
            &params.redirect
        };
        axum::response::Redirect::to(redirect_to)
    } else {
        // In production, show error message
        let redirect_param = if params.redirect.is_empty() {
            String::new()
        } else {
            format!("?redirect={}", urlencoding::encode(&params.redirect))
        };
        axum::response::Redirect::to(&format!("/login{}", redirect_param))
    }
}

pub async fn logout_handler(mut auth_session: AuthSession) -> axum::response::Redirect {
    let _ = auth_session.logout().await;
    axum::response::Redirect::to("/")
}
