#[derive(Clone, Debug)]
pub struct Web3ApiManifest {
    pub format: String,
    pub repository: Option<String>,
    pub build: Option<String>,
    pub language: Option<String>,
    pub interface: Option<bool>,
    pub modules: Modules,
    pub import_redirects: Option<Vec<ImportRedirects>>,
    pub __type: String,
}

impl Default for Web3ApiManifest {
    fn default() -> Web3ApiManifest {
        Web3ApiManifest {
            format: "0.0.1-prealpha.3".to_string(),
            repository: None,
            build: None,
            language: None,
            interface: None,
            modules: Modules::default(),
            import_redirects: None,
            __type: "Web3ApiManifest".to_string(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Modules {
    pub mutation: Option<Mutation>,
    pub query: Option<Query>,
}

impl Default for Modules {
    fn default() -> Modules {
        Modules {
            mutation: None,
            query: None,
        }
    }
}

#[derive(Clone, Debug)]
pub struct Mutation {
    pub schema: Schema,
    pub module: Module,
}

#[derive(Clone, Debug)]
pub struct Query {
    pub schema: Schema,
    pub module: Module,
}

#[derive(Clone, Debug)]
pub struct Module {
    pub language: String,
    pub file: String,
}

#[derive(Clone, Debug)]
pub struct Schema {
    pub file: String,
}

#[derive(Clone, Debug)]
pub struct ImportRedirects {
    pub uri: String,
    pub schema: String,
}