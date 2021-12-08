pub mod deserialize;
pub mod migrate;
pub mod prealpha_001_1;
pub mod validate;

pub use migrate::migrate_meta_manifest;
pub use prealpha_001_1::MetaManifest as MetaManifest001Prealpha1;

pub type MetaManifest = MetaManifest001Prealpha1;

pub const LATEST_META_MANIFEST_FORMAT: &'static str = MetaManifestFormats::PREALPHA_001_1;

#[non_exhaustive]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MetaManifestFormats;

impl MetaManifestFormats {
    pub const PREALPHA_001_1: &'static str = "0.0.1-prealpha.1";

    pub fn get_format_name(input: &str) -> Option<&str> {
        let format = match input {
            MetaManifestFormats::PREALPHA_001_1 => MetaManifestFormats::PREALPHA_001_1,
            _ => "",
        };
        if format.is_empty() {
            return None;
        }
        Some(format)
    }
}

pub enum AnyMetaManifest {
    MetaManifest001Prealpha1(MetaManifest001Prealpha1),
}

impl AnyMetaManifest {
    pub fn get_manifest_format(manifest: &mut Self) -> &String {
        match manifest {
            AnyMetaManifest::MetaManifest001Prealpha1(one) => &one.format,
        }
    }
}

#[derive(Clone, Debug)]
pub struct Links {
    pub name: String,
    pub icon: Option<String>,
    pub url: String,
}

#[derive(Clone, Debug)]
pub struct Queries {
    pub name: String,
    pub description: Option<String>,
    pub query: String,
    pub vars: Option<String>,
}
