#[derive(Debug, Clone)]
pub struct DeepLinkConfig {
    pub scheme: String,
    pub domains: Vec<String>,
}

impl DeepLinkConfig {
    pub fn new(scheme: impl Into<String>) -> Self {
        Self {
            scheme: scheme.into(),
            domains: Vec::new(),
        }
    }

    pub fn domain(mut self, domain: impl Into<String>) -> Self {
        self.domains.push(domain.into());
        self
    }
}
