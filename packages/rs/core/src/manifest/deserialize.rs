#[derive(Debug, Default)]
pub struct DeserializeManifestOptions {
    pub no_validate: Option<bool>,
    pub ext_schema: Option<serde_json::Value>,
}
