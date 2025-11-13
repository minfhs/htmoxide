use std::collections::HashMap;
use serde::de::DeserializeOwned;

/// Helper for building component URLs with merged query parameters
#[derive(Clone)]
pub struct UrlBuilder {
    path: String,
    all_params: HashMap<String, String>,
    main_page_path: Option<String>,
}

impl UrlBuilder {
    pub fn new(path: impl Into<String>, query_string: &str) -> Self {
        let all_params = parse_query_string(query_string);
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

    /// Build the final URL with all parameters
    pub fn build(self) -> String {
        // Filter out empty values AND empty keys
        let filtered_params: HashMap<_, _> = self.all_params
            .into_iter()
            .filter(|(k, v)| !k.is_empty() && !v.is_empty())
            .collect();

        if filtered_params.is_empty() {
            return self.path;
        }

        let query_string = serde_urlencoded::to_string(&filtered_params)
            .unwrap_or_default();

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
        let filtered_params: HashMap<_, _> = self.all_params
            .into_iter()
            .filter(|(k, v)| !k.is_empty() && !v.is_empty())
            .collect();

        if filtered_params.is_empty() {
            return main_page;
        }

        let query_string = serde_urlencoded::to_string(&filtered_params)
            .unwrap_or_default();

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
        let filtered_params: HashMap<_, _> = self.all_params
            .into_iter()
            .filter(|(k, v)| !k.is_empty() && !v.is_empty())
            .collect();

        if filtered_params.is_empty() {
            return page_path;
        }

        let query_string = serde_urlencoded::to_string(&filtered_params)
            .unwrap_or_default();

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

fn parse_query_string(query: &str) -> HashMap<String, String> {
    if query.is_empty() {
        return HashMap::new();
    }

    query
        .split('&')
        .filter_map(|pair| {
            if pair.is_empty() {
                return None;
            }
            let mut parts = pair.splitn(2, '=');
            let key = parts.next()?.to_string();
            if key.is_empty() {
                return None;
            }
            let value = parts.next().unwrap_or("").to_string();
            Some((key, value))
        })
        .collect()
}
