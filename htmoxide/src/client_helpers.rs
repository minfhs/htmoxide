use maud::{html, Markup, PreEscaped};
use std::collections::HashMap;

/// Returns a script tag that clears cookies for empty parameter values.
/// 
/// This solves the common problem where browsers don't send empty form values,
/// which would cause old cookie values to persist. The script listens for htmx
/// requests and clears cookies client-side when parameters are sent as empty strings.
/// 
/// Include this in your HTML head after loading htmx:
/// 
/// ```rust
/// use htmoxide::prelude::*;
/// use htmoxide::cookie_cleaner_script;
/// 
/// html! {
///     head {
///         script src="https://unpkg.com/htmx.org@1.9.10" {}
///         (cookie_cleaner_script())
///     }
/// }
/// ```
pub fn cookie_cleaner_script() -> Markup {
    html! {
        script {
            (PreEscaped(r#"
            // htmoxide: Clear cookies client-side when parameters are empty
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
    }
}

/// Renders hidden input fields to preserve URL parameters.
/// 
/// Useful in forms that need to maintain other component state while updating one parameter.
/// This prevents state loss when only part of the URL parameters are being updated.
/// 
/// # Arguments
/// * `params` - HashMap of all current URL parameters
/// * `exclude` - Slice of parameter names to exclude (typically the ones being actively edited)
/// 
/// # Example
/// ```rust
/// use htmoxide::prelude::*;
/// use htmoxide::preserve_params;
/// 
/// // In a component that edits "filter" but wants to preserve "sort", "count", etc.
/// html! {
///     form {
///         input type="text" name="filter" value=(state.filter);
///         (preserve_params(&all_params, &["filter"]))
///         button { "Submit" }
///     }
/// }
/// ```
pub fn preserve_params(params: &HashMap<String, String>, exclude: &[&str]) -> Markup {
    html! {
        @for (key, value) in params {
            @if !exclude.contains(&key.as_str()) && !value.is_empty() {
                input type="hidden" name=(key) value=(value);
            }
        }
    }
}

/// Generates a JavaScript onclick handler to clear an input and trigger htmx.
/// 
/// Useful for "Clear" buttons that need to clear a text input and immediately
/// trigger an htmx request to update the UI.
/// 
/// # Arguments
/// * `input_id` - The DOM ID of the input element to clear
/// * `event` - The event to trigger (typically "keyup" or "change")
/// 
/// # Example
/// ```rust
/// use htmoxide::clear_input_handler;
/// 
/// html! {
///     button onclick=(clear_input_handler("search-input", "keyup")) {
///         "Clear"
///     }
/// }
/// ```
pub fn clear_input_handler(input_id: &str, event: &str) -> String {
    format!(
        "document.getElementById('{input_id}').value = ''; htmx.trigger('#{input_id}', '{event}');"
    )
}
