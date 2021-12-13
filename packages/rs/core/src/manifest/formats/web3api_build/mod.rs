use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub mod deserialize;
pub mod migrate;
pub mod migrators;
pub mod prealpha_001_1;
pub mod prealpha_001_2;
pub mod validate;

pub use migrate::migrate_build_manifest;
pub use prealpha_001_1::BuildManifest as BuildManifest001Prealpha1;
pub use prealpha_001_2::BuildManifest as BuildManifest001Prealpha2;
pub use validate::validate_build_manifest;

pub type BuildManifest<T> = BuildManifest001Prealpha2<T>;

pub const LATEST_BUILD_MANIFEST_FORMAT: &'static str = BuildManifestFormats::PREALPHA_001_2;

#[non_exhaustive]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct BuildManifestFormats;

impl BuildManifestFormats {
    pub const PREALPHA_001_1: &'static str = "0.0.1-prealpha.1";
    pub const PREALPHA_001_2: &'static str = "0.0.1-prealpha.2";

    pub fn get_format_name(input: &str) -> Option<&str> {
        let format = match input {
            BuildManifestFormats::PREALPHA_001_1 => BuildManifestFormats::PREALPHA_001_1,
            BuildManifestFormats::PREALPHA_001_2 => BuildManifestFormats::PREALPHA_001_2,
            _ => "",
        };
        if format.is_empty() {
            return None;
        }
        Some(format)
    }
}

#[derive(Debug, Clone)]
pub enum AnyBuildManifest<T: Clone + std::fmt::Debug + Serialize> {
    BuildManifest001Prealpha1(BuildManifest001Prealpha1<T>),
    BuildManifest001Prealpha2(BuildManifest001Prealpha2<T>),
}

impl<T: Clone + std::fmt::Debug + Serialize> AnyBuildManifest<T> {
    pub fn get_manifest_format(manifest: &Self) -> &String {
        match manifest {
            AnyBuildManifest::BuildManifest001Prealpha1(one) => &one.format,
            AnyBuildManifest::BuildManifest001Prealpha2(two) => &two.format,
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Docker {
    pub name: Option<String>,
    pub dockerfile: Option<String>,
    pub build_image_id: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Config<T: Clone> {
    pub k: HashMap<String, T>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct LinkedPackages {
    pub name: String,
    pub path: String,
    pub filter: Option<String>,
}
