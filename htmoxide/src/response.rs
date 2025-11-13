use axum::{
    response::{IntoResponse, Response},
    http::{StatusCode, HeaderValue},
};
use maud::{Markup, Render};

/// Response type for component partial renders
#[derive(Debug, Clone)]
pub struct Html {
    pub markup: Markup,
    pub push_url: Option<String>,
}

impl From<Markup> for Html {
    fn from(markup: Markup) -> Self {
        Html { markup, push_url: None }
    }
}

impl Html {
    pub fn new(markup: Markup) -> Self {
        Html { markup, push_url: None }
    }

    pub fn with_push_url(mut self, url: String) -> Self {
        self.push_url = Some(url);
        self
    }
}

impl Render for Html {
    fn render(&self) -> Markup {
        self.markup.clone()
    }
}

impl IntoResponse for Html {
    fn into_response(self) -> Response {
        let mut response = (
            StatusCode::OK,
            [("Content-Type", "text/html; charset=utf-8")],
            self.markup.into_string(),
        )
            .into_response();

        // Add HX-Push-Url header if specified
        if let Some(push_url) = self.push_url {
            if let Ok(header_value) = HeaderValue::from_str(&push_url) {
                response.headers_mut().insert("HX-Push-Url", header_value);
            }
        }

        response
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
