/// Global configuration for htmoxide.
///
/// Initialize once at application startup with [`configure`]:
///
/// ```ignore
/// htmoxide::configure(HtmoxideConfig::new().fragment_prefix("/x"));
/// ```
pub struct HtmoxideConfig {
    pub fragment_prefix: Option<&'static str>,
}

impl HtmoxideConfig {
    pub fn new() -> Self {
        Self {
            fragment_prefix: None,
        }
    }

    /// Set the fragment routing prefix. All htmx component endpoints should begin
    /// with this prefix. It will be stripped to produce the browser-visible push URL.
    pub fn fragment_prefix(mut self, prefix: &'static str) -> Self {
        self.fragment_prefix = Some(prefix);
        self
    }
}

impl Default for HtmoxideConfig {
    fn default() -> Self {
        Self::new()
    }
}

static CONFIG: std::sync::OnceLock<HtmoxideConfig> = std::sync::OnceLock::new();

/// Initialize htmoxide with the given configuration.
///
/// Call once at application startup, before serving any requests.
/// Subsequent calls are silently ignored (the first call wins).
pub fn configure(config: HtmoxideConfig) {
    let _ = CONFIG.set(config);
}

/// Returns the configured fragment prefix, if any.
pub(crate) fn fragment_prefix() -> Option<&'static str> {
    CONFIG.get().and_then(|c| c.fragment_prefix)
}
