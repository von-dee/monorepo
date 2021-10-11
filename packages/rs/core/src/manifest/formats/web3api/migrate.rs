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

pub type MigratorMap = HashMap<
    &'static str,
    fn(manifest: &mut AnyWeb3ApiManifest) -> Result<Web3ApiManifest, &'static str>,
>;
pub struct Migrator;
impl Migrator {
    // pub fn new() -> MigratorMap {
    //     let mut migrators: MigratorMap = HashMap::new();
    //     let _ = migrators.insert("0.0.1-prealpha.1", migrate_prealpha_001_1_to_prealpha_001_5);
    //     todo!()
    // }
}

pub fn migrate_web3_api_manifest(
    manifest: &mut AnyWeb3ApiManifest,
    to: &mut Web3ApiManifestFormats,
) -> Result<Web3ApiManifest, String> {
    let from = manifest.format;

    if from == LATEST_WEB3_API_MANIFEST_FORMAT {
        return Ok(Web3ApiManifest {
            format: "0.0.1-prealpha.5".to_string(),
            build: manifest.build.clone(),
            meta: manifest.meta.clone(),
            language: manifest.language.clone(),
            modules: manifest.modules.clone().unwrap(),
            import_redirects: manifest.import_redirects.clone(),
            __type: manifest.__type.clone(),
        });
    }

    match from {
        Web3ApiManifestFormats::Prealpha001_1("0.0.1-prealpha.1")
        | Web3ApiManifestFormats::Prealpha001_2("0.0.1-prealpha.2")
        | Web3ApiManifestFormats::Prealpha001_3("0.0.1-prealpha.3")
        | Web3ApiManifestFormats::Prealpha001_4("0.0.1-prealpha.4")
        | Web3ApiManifestFormats::Prealpha001_5("0.0.1-prealpha.5") => {}
        _ => {}
    }

    todo!()
}
