use super::{ImportRedirects, Modules};

#[derive(Clone, Debug)]
pub struct Web3ApiManifest {
    pub format: String,
    pub repository: Option<String>,
    pub build: Option<String>,
    pub language: Option<String>,
    pub interface: bool,
    pub modules: Modules,
    pub import_redirects: Option<Vec<ImportRedirects>>,
    pub __type: String,
}

impl Default for Web3ApiManifest {
    fn default() -> Web3ApiManifest {
        Web3ApiManifest {
            format: "0.0.1-prealpha.3".to_string(),
            repository: None,
            build: None,
            language: None,
            interface: false,
            modules: Modules::default(),
            import_redirects: None,
            __type: "Web3ApiManifest".to_string(),
        }
    }
}
