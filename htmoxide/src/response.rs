use axum::{
    response::{IntoResponse, Response},
    http::StatusCode,
};
use maud::{Markup, Render};

/// Response type for component partial renders
#[derive(Debug, Clone)]
pub struct Html(pub Markup);

impl From<Markup> for Html {
    fn from(markup: Markup) -> Self {
        Html(markup)
    }
}

impl Render for Html {
    fn render(&self) -> Markup {
        self.0.clone()
    }
}

impl IntoResponse for Html {
    fn into_response(self) -> Response {
        (
            StatusCode::OK,
            [("Content-Type", "text/html; charset=utf-8")],
            self.0.into_string(),
        )
            .into_response()
    }
}

/// Response type for full page renders
#[derive(Debug, Clone)]
pub struct Page(pub Markup);

impl From<Markup> for Page {
    fn from(markup: Markup) -> Self {
        Page(markup)
    }
}

impl IntoResponse for Page {
    fn into_response(self) -> Response {
        // Wrap in full HTML document
        let full_html = maud::html! {
            (maud::DOCTYPE)
            html {
                (self.0)
            }
        };

        (
            StatusCode::OK,
            [("Content-Type", "text/html; charset=utf-8")],
            full_html.into_string(),
        )
            .into_response()
    }
}
