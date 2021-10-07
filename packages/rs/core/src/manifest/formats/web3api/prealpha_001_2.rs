use super::{ImportRedirects, Modules};

#[derive(Clone, Debug)]
pub struct Web3ApiManifest {
    pub format: String,
    pub repository: Option<String>,
    pub build: Option<String>,
    pub language: String,
    pub modules: Modules,
    pub import_redirects: Option<Vec<ImportRedirects>>,
    pub __type: String,
}

impl Default for Web3ApiManifest {
    fn default() -> Web3ApiManifest {
        Web3ApiManifest {
            format: "0.0.1-prealpha.2".to_string(),
            repository: None,
            build: None,
            language: "".to_string(),
            modules: Modules::default(),
            import_redirects: None,
            __type: "Web3ApiManifest".to_string(),
        }
    }
}
