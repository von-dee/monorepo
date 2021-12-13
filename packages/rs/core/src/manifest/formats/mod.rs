pub mod web3api;
pub mod web3api_build;
pub mod web3api_meta;
pub mod web3api_plugin;

use web3api::Web3ApiManifest;
use web3api_build::BuildManifest;
use web3api_meta::MetaManifest;

#[non_exhaustive]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct ManifestType;

impl ManifestType {
    pub const WEB3API: &'static str = "web3api";
    pub const META: &'static str = "meta";
    pub const BUILD: &'static str = "build";
}

#[derive(Debug, Clone)]
pub enum AnyManifest<T: Clone + std::fmt::Debug + serde::Serialize> {
    Web3ApiManifest(Web3ApiManifest),
    BuildManifest(BuildManifest<T>),
    MetaManifest(MetaManifest),
}
