use super::{Config, Docker, LinkedPackages};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct BuildManifest {
    pub format: String,
    pub docker: Option<Docker>,
    pub config: Option<Config>,
    pub linked_packages: Option<Vec<LinkedPackages>>,
    pub __type: String,
}

impl Default for BuildManifest {
    fn default() -> BuildManifest {
        BuildManifest {
            format: "0.0.1-prealpha.2".to_string(),
            docker: None,
            config: None,
            linked_packages: None,
            __type: "BuildManifest".to_string(),
        }
    }
}
