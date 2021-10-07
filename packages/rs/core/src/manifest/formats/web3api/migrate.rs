use std::collections::HashMap;

use super::migrators::{
    prealpha_001_1_to_prealpha_001_5::migrate as migrate_prealpha_001_1_to_prealpha_001_5,
    prealpha_001_2_to_prealpha_001_5::migrate as migrate_prealpha_001_2_to_prealpha_001_5,
    prealpha_001_3_to_prealpha_001_5::migrate as migrate_prealpha_001_3_to_prealpha_001_5,
    prealpha_001_4_to_prealpha_001_5::migrate as migrate_prealpha_001_4_to_prealpha_001_5,
};
use super::{
    AnyWeb3ApiManifest, Web3ApiManifest, Web3ApiManifestFormats, LATEST_WEB3_API_MANIFEST_FORMAT,
};

pub type Migrator = HashMap<&'static str, fn(old: &mut AnyWeb3ApiManifest) -> Web3ApiManifest>;

pub fn migrate_web3_api_manifest(
    manifest: &mut AnyWeb3ApiManifest,
    to: &mut Web3ApiManifestFormats,
) -> Result<Web3ApiManifest, String> {
    todo!()
}
