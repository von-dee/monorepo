use super::{Config, Docker};

#[derive(Clone, Debug)]
pub struct BuildManifest<T: Clone> {
    pub format: String,
    pub docker: Option<Docker>,
    pub config: Option<Config<T>>,
    pub __type: String,
}

impl<T: Clone> Default for BuildManifest<T> {
    fn default() -> BuildManifest<T> {
        BuildManifest {
            format: "0.0.1-prealpha.1".to_string(),
            docker: None,
            config: None,
            __type: "BuildManifest".to_string(),
        }
    }
}
