pub mod deserialize;
pub mod migrate;
pub mod migrators;
pub mod prealpha_001_1;
pub mod prealpha_001_2;
pub mod prealpha_001_3;
pub mod prealpha_001_4;
pub mod prealpha_001_5;
pub mod validate;

pub use migrate::migrate_web3_api_manifest;
pub use prealpha_001_1::Web3ApiManifest as Web3ApiManifest001Prealpha1;
pub use prealpha_001_2::Web3ApiManifest as Web3ApiManifest001Prealpha2;
pub use prealpha_001_3::Web3ApiManifest as Web3ApiManifest001Prealpha3;
pub use prealpha_001_4::Web3ApiManifest as Web3ApiManifest001Prealpha4;
pub use prealpha_001_5::Web3ApiManifest as Web3ApiManifest001Prealpha5;

pub type Web3ApiManifest = Web3ApiManifest001Prealpha5;

pub const LATEST_WEB3_API_MANIFEST_FORMAT: &'static str = Web3ApiManifestFormats::PREALPHA_001_5;

#[non_exhaustive]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Web3ApiManifestFormats;

impl Web3ApiManifestFormats {
    pub const PREALPHA_001_1: &'static str = "0.0.1-prealpha.1";
    pub const PREALPHA_001_2: &'static str = "0.0.1-prealpha.2";
    pub const PREALPHA_001_3: &'static str = "0.0.1-prealpha.3";
    pub const PREALPHA_001_4: &'static str = "0.0.1-prealpha.4";
    pub const PREALPHA_001_5: &'static str = "0.0.1-prealpha.5";

    pub fn get_format_name(input: &str) -> Option<&str> {
        let format = match input {
            Web3ApiManifestFormats::PREALPHA_001_1 => Web3ApiManifestFormats::PREALPHA_001_1,
            Web3ApiManifestFormats::PREALPHA_001_2 => Web3ApiManifestFormats::PREALPHA_001_2,
            Web3ApiManifestFormats::PREALPHA_001_3 => Web3ApiManifestFormats::PREALPHA_001_3,
            Web3ApiManifestFormats::PREALPHA_001_4 => Web3ApiManifestFormats::PREALPHA_001_4,
            Web3ApiManifestFormats::PREALPHA_001_5 => Web3ApiManifestFormats::PREALPHA_001_5,
            _ => "",
        };
        if format.is_empty() {
            return None;
        }
        Some(format)
    }
}

pub enum AnyWeb3ApiManifest {
    Web3ApiManifest001Prealpha1(Web3ApiManifest001Prealpha1),
    Web3ApiManifest001Prealpha2(Web3ApiManifest001Prealpha2),
    Web3ApiManifest001Prealpha3(Web3ApiManifest001Prealpha3),
    Web3ApiManifest001Prealpha4(Web3ApiManifest001Prealpha4),
    Web3ApiManifest001Prealpha5(Web3ApiManifest001Prealpha5),
}

impl AnyWeb3ApiManifest {
    pub fn get_manifest_format(manifest: &mut Self) -> &String {
        match manifest {
            AnyWeb3ApiManifest::Web3ApiManifest001Prealpha1(one) => &one.format,
            AnyWeb3ApiManifest::Web3ApiManifest001Prealpha2(two) => &two.format,
            AnyWeb3ApiManifest::Web3ApiManifest001Prealpha3(three) => &three.format,
            AnyWeb3ApiManifest::Web3ApiManifest001Prealpha4(four) => &four.format,
            AnyWeb3ApiManifest::Web3ApiManifest001Prealpha5(five) => &five.format,
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
