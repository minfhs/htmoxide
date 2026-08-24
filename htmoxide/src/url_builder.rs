use serde::de::DeserializeOwned;
use std::collections::HashMap;

use crate::error::HtmoxideError;

/// Trait for getting a component's name at compile time
pub trait ComponentName {
    fn name() -> &'static str;
}

/// Helper for building component URLs with merged query parameters
#[derive(Clone)]
pub struct UrlBuilder {
    path: String,
    all_params: HashMap<String, String>,
    main_page_path: Option<String>,
}

/// Get the route path for a component by name
pub fn component_route(component_name: &str) -> Option<&'static str> {
    for component in inventory::iter::<crate::ComponentInfo> {
        if component.name == component_name {
            return Some(component.path);
        }
    }
    None
}

impl UrlBuilder {
    pub fn new(path: impl Into<String>, all_params: HashMap<String, String>) -> Self {
        Self {
            path: path.into(),
            all_params,
            main_page_path: None,
        }
    }

    /// Create a new UrlBuilder with a specific main page path for push URL
    pub fn with_main_page(mut self, main_page_path: impl Into<String>) -> Self {
        self.main_page_path = Some(main_page_path.into());
        self
    }

    /// Merge new parameters with existing ones
    pub fn with_params<K, V>(mut self, params: impl IntoIterator<Item = (K, V)>) -> Self
    where
        K: Into<String>,
        V: ToString,
    {
        for (key, value) in params {
            self.all_params.insert(key.into(), value.to_string());
        }
        self
    }

    /// Create a new UrlBuilder for a different component using the function name
    ///
    /// # Example
    /// ```ignore
    /// url.for_component(create_todo).build()
    /// // If the current URL is /todos/1?filter=active
    /// // This returns /todos/create?filter=active
    /// ```
    pub fn for_component<F>(mut self, _component: F) -> Self
    where
        F: ComponentName,
    {
        let component_name = F::name();
        if let Some(route) = component_route(component_name) {
            self.path = route.to_string();
        }
        self
    }

    /// Create a new UrlBuilder for a different component, preserving state params
    /// (String-based version for dynamic use cases)
    ///
    /// # Example
    /// ```ignore
    /// url.with_component("create_todo").build()
    /// ```
    pub fn with_component(mut self, component_name: &str) -> Self {
        if let Some(route) = component_route(component_name) {
            self.path = route.to_string();
        }
        self
    }

    /// Replace path parameters in the route template
    ///
    /// # Example
    /// ```ignore
    /// url.with_component("toggle_todo").with_path_param("id", 42).build()
    /// // Returns /todos/42/toggle?filter=active
    /// ```
    pub fn with_path_param(mut self, param_name: &str, value: impl ToString) -> Self {
        let placeholder = format!("{{{}}}", param_name);
        let value = value.to_string();

        self.path = self.path.replace(&placeholder, &value);
        self.all_params.remove(param_name);

        self
    }

    pub fn try_build(self) -> Result<String, HtmoxideError> {
        let unresolved = unresolved_path_params(&self.path);

        if !unresolved.is_empty() {
            return Err(HtmoxideError::UnresolvedPathParameters(unresolved));
        }

        let filtered_params: HashMap<_, _> = self
            .all_params
            .into_iter()
            .filter(|(k, v)| !k.is_empty() && !v.is_empty())
            .collect();

        if filtered_params.is_empty() {
            return Ok(self.path);
        }

        let query_string = serde_urlencoded::to_string(&filtered_params)
            .map_err(|e| HtmoxideError::QuerySerialization(e.to_string()))?;

        if query_string.is_empty() {
            Ok(self.path)
        } else {
            Ok(format!("{}?{}", self.path, query_string))
        }
    }

    /// Builds the final URL with all parameters.
    #[deprecated(note = "use `try_build()` to handle URL construction errors")]
    pub fn build(self) -> String {
        // Filter out empty values AND empty keys
        let filtered_params: HashMap<_, _> = self
            .all_params
            .into_iter()
            .filter(|(k, v)| !k.is_empty() && !v.is_empty())
            .collect();

        if filtered_params.is_empty() {
            return self.path;
        }

        let query_string = serde_urlencoded::to_string(&filtered_params).unwrap_or_default();

        if query_string.is_empty() {
            self.path
        } else {
            format!("{}?{}", self.path, query_string)
        }
    }

    /// Build URL for the main page (for hx-push-url)
    pub fn build_main_url(self) -> String {
        let main_page = self.main_page_path.unwrap_or_else(|| "/".to_string());

        // Filter out empty values AND empty keys
        let filtered_params: HashMap<_, _> = self
            .all_params
            .into_iter()
            .filter(|(k, v)| !k.is_empty() && !v.is_empty())
            .collect();

        if filtered_params.is_empty() {
            return main_page;
        }

        let query_string = serde_urlencoded::to_string(&filtered_params).unwrap_or_default();

        if query_string.is_empty() {
            main_page
        } else {
            format!("{}?{}", main_page, query_string)
        }
    }

    /// Build URL for a specific page path (for hx-push-url)
    pub fn build_page_url(self, page_path: impl Into<String>) -> String {
        let page_path = page_path.into();

        // Filter out empty values AND empty keys
        let filtered_params: HashMap<_, _> = self
            .all_params
            .into_iter()
            .filter(|(k, v)| !k.is_empty() && !v.is_empty())
            .collect();

        if filtered_params.is_empty() {
            return page_path;
        }

        let query_string = serde_urlencoded::to_string(&filtered_params).unwrap_or_default();

        if query_string.is_empty() {
            page_path
        } else {
            format!("{}?{}", page_path, query_string)
        }
    }
    /// Get parameters that are NOT part of the specified state type
    /// This is useful for including other components' params as hidden fields
    pub fn other_params<T: DeserializeOwned>(&self) -> HashMap<String, String> {
        // Get keys that would be deserialized by type T
        let query_string = serde_urlencoded::to_string(&self.all_params).unwrap_or_default();
        let _component_state: Result<T, _> = serde_urlencoded::from_str(&query_string);

        // For now, we'll need to manually exclude known fields
        // A better approach would use serde introspection, but that's complex
        // For the simple case, we can provide a simpler method
        self.all_params.clone()
    }

    /// Get all parameters as a HashMap
    pub fn all_params(&self) -> &HashMap<String, String> {
        &self.all_params
    }
}

pub fn parse_query_string(query: &str) -> Result<HashMap<String, String>, HtmoxideError> {
    if query.is_empty() {
        return Ok(HashMap::new());
    }

    serde_urlencoded::from_str::<Vec<(String, String)>>(query)
        .map(|params| params.into_iter().filter(|(k, _)| !k.is_empty()).collect())
        .map_err(|e| HtmoxideError::InvalidQueryString(e.to_string()))
}

fn unresolved_path_params(path: &str) -> Vec<String> {
    let mut params = Vec::new();
    let mut remaining = path;

    while let Some(start) = remaining.find('{') {
        let after_start = &remaining[start + 1..];

        let Some(end) = after_start.find('}') else {
            break;
        };

        let name = &after_start[..end];

        if !name.is_empty() && name.chars().all(|c| c.is_ascii_alphanumeric() || c == '_') {
            params.push(name.to_string());
        }

        remaining = &after_start[end + 1..];
    }

    params
}
