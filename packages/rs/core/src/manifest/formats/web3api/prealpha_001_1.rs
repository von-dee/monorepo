#[derive(Clone, Debug)]
pub struct Web3ApiManifest {
    format: String,
    description: Option<String>,
    repository: Option<String>,
    mutation: Option<Mutation>,
    query: Option<Query>,
    import_redirects: Option<Vec<ImportRedirects>>,
    __type: String,
}

impl Default for Web3ApiManifest {
    fn default() -> Web3ApiManifest {
        Web3ApiManifest {
            format: "0.0.1-prealpha.1".to_string(),
            description: None,
            repository: None,
            mutation: None,
            query: None,
            import_redirects: None,
            __type: "Web3ApiManifest".to_string(),
        }
    }
}

#[derive(Clone, Debug)]
enum Mutation {
    Schema { file: String },
    Module { language: String, file: String },
}

#[derive(Clone, Debug)]
enum Query {
    Schema { file: String },
    Module { language: String, file: String },
}

#[derive(Clone, Debug)]
struct ImportRedirects {
    uri: String,
    schema: String,
}
