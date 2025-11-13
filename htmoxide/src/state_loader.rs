use serde::de::DeserializeOwned;
use tower_cookies::Cookies;
use std::collections::HashMap;

/// Sentinel value to explicitly unset a field (clear it to empty)
pub const UNSET_SENTINEL: &str = "__HTMOXIDE_UNSET__";

/// Helper for loading component state from cookies and URL parameters
/// 
/// This handles the common pattern of:
/// 1. Load default state
/// 2. Override with values from cookies
/// 3. Override with values from URL query params (highest priority)
pub struct StateLoader {
    cookies: Cookies,
    query_params: HashMap<String, String>,
}

impl StateLoader {
    /// Create a new StateLoader from cookies and query parameters
    pub fn new(cookies: Cookies, query_params: HashMap<String, String>) -> Self {
        Self {
            cookies,
            query_params,
        }
    }

    /// Load state with cookie fallback and URL override
    /// 
    /// Priority order (highest to lowest):
    /// 1. URL query parameters (bookmarkable)
    /// 2. Cookies (persistence)
    /// 3. Default values
    pub fn load<T>(&self) -> T
    where
        T: DeserializeOwned + Default + serde::Serialize,
    {
        // Start with default state
        let mut state = T::default();
        
        // Try to serialize to JSON to access individual fields
        if let Ok(mut state_json) = serde_json::to_value(&state) {
            if let Some(state_obj) = state_json.as_object_mut() {
                if let Ok(default_json) = serde_json::to_value(&T::default()) {
                    if let Some(default_obj) = default_json.as_object() {
                        // For each field, check cookies first, then query params
                        for (key, default_value) in default_obj {
                            let mut current_value = default_value.clone();
                            
                            // First, try to load from cookie
                            if let Some(cookie) = self.cookies.get(key) {
                                let cookie_value = cookie.value();
                                if let Some(parsed) = Self::parse_value(cookie_value) {
                                    current_value = parsed;
                                }
                            }
                            
                            // Then, override with query param if present
                            if let Some(query_value) = self.query_params.get(key) {
                                if query_value == UNSET_SENTINEL {
                                    // Explicitly unset - use empty string
                                    current_value = serde_json::Value::String(String::new());
                                } else if let Some(parsed) = Self::parse_value(query_value) {
                                    current_value = parsed;
                                }
                            }
                            
                            state_obj.insert(key.clone(), current_value);
                        }
                    }
                }
                
                // Deserialize back to state
                if let Ok(new_state) = serde_json::from_value(state_json) {
                    state = new_state;
                }
            }
        }
        
        state
    }
    
    /// Parse a string value into a JSON value
    fn parse_value(value: &str) -> Option<serde_json::Value> {
        if let Ok(num) = value.parse::<i64>() {
            Some(serde_json::Value::Number(num.into()))
        } else if let Ok(num) = value.parse::<f64>() {
            serde_json::Number::from_f64(num).map(serde_json::Value::Number)
        } else if let Ok(b) = value.parse::<bool>() {
            Some(serde_json::Value::Bool(b))
        } else {
            // Always return a string, even if empty
            // Empty strings are valid values that should override cookies
            Some(serde_json::Value::String(value.to_string()))
        }
    }
}
