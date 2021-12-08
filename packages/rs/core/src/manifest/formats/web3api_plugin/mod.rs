pub mod deserialize;
pub mod migrate;
pub mod prealpha_001_1;
pub mod prealpha_001_3;
pub mod validate;

pub use migrate::migrate_plugin_manifest;
pub use prealpha_001_1::PluginManifest as PluginManifest001Prealpha1;

pub type PluginManifest = PluginManifest001Prealpha1;

pub const LATEST_PLUGIN_MANIFEST_FORMAT: &'static str = PluginManifestFormats::PREALPHA_001_1;

#[non_exhaustive]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct PluginManifestFormats;

impl PluginManifestFormats {
    pub const PREALPHA_001_1: &'static str = "0.0.1-prealpha.1";

    pub fn get_format_name(input: &str) -> Option<&str> {
        let format = match input {
            PluginManifestFormats::PREALPHA_001_1 => PluginManifestFormats::PREALPHA_001_1,
            _ => "",
        };
        if format.is_empty() {
            return None;
        }
        Some(format)
    }
}

pub enum AnyPluginManifest {
    PluginManifest001Prealpha1(PluginManifest001Prealpha1),
}

impl AnyPluginManifest {
    pub fn get_manifest_format(manifest: &mut Self) -> &String {
        match manifest {
            AnyPluginManifest::PluginManifest001Prealpha1(one) => &one.format,
        }
    }
}

#[derive(Clone, Debug)]
pub struct ImportRedirects {
    pub uri: String,
    pub schema: String,
}
