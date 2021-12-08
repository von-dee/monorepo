use super::{Links, Queries};

#[derive(Clone, Debug)]
pub struct MetaManifest {
    pub format: String,
    pub name: String,
    pub subtext: Option<String>,
    pub description: Option<String>,
    pub repository: Option<String>,
    pub icon: Option<String>,
    pub links: Option<Vec<Links>>,
    pub queries: Option<Vec<Queries>>,
    pub __type: String,
}

impl Default for MetaManifest {
    fn default() -> MetaManifest {
        MetaManifest {
            format: "0.0.1-prealpha.1".to_string(),
            name: "".to_string(),
            subtext: None,
            description: None,
            repository: None,
            icon: None,
            links: None,
            queries: None,
            __type: "MetaManifest".to_string(),
        }
    }
}
