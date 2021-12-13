use super::{ImportRedirects, Module, Mutation, Query};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Web3ApiManifest {
    pub format: String,
    pub description: Option<String>,
    pub repository: Option<String>,
    pub mutation: Option<Mutation>,
    pub query: Option<Query>,
    pub module: Option<Module>,
    pub import_redirects: Option<Vec<ImportRedirects>>,
    pub __type: String,
}

impl Default for Web3ApiManifest {
    fn default() -> Web3ApiManifest {
        Web3ApiManifest {
            format: "0.0.1-prealpha.1".to_string(),
            description: None,
            repository: None,
            mutation: None,
            query: None,
            module: None,
            import_redirects: None,
            __type: "Web3ApiManifest".to_string(),
        }
    }
}
