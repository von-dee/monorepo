use valico::json_schema::Schema as JsonSchema;

#[derive(Debug, Default)]
pub struct DeserializeManifestOptions {
    pub no_validate: Option<bool>,
    pub ext_schema: Option<JsonSchema>,
}
