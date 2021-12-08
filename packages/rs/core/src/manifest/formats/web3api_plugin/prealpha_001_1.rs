use super::ImportRedirects;

#[derive(Clone, Debug)]
pub struct PluginManifest {
    pub format: String,
    pub language: String,
    pub schema: String,
    pub import_redirects: Option<Vec<ImportRedirects>>,
    pub __type: String,
}

impl Default for PluginManifest {
    fn default() -> PluginManifest {
        PluginManifest {
            format: "0.0.1-prealpha.1".to_string(),
            language: "".to_string(),
            schema: "".to_string(),
            import_redirects: None,
            __type: "PluginManifest".to_string(),
        }
    }
}
