use super::{Config, Docker};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct BuildManifest {
    pub format: String,
    pub docker: Option<Docker>,
    pub config: Option<Config>,
    pub __type: String,
}

impl Default for BuildManifest {
    fn default() -> BuildManifest {
        BuildManifest {
            format: "0.0.1-prealpha.1".to_string(),
            docker: None,
            config: None,
            __type: "BuildManifest".to_string(),
        }
    }
}
