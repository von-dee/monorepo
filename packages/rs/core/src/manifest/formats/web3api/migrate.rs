use super::migrators::{
    prealpha_001_1_to_prealpha_001_5::migrate as migrate_prealpha_001_1_to_prealpha_001_5,
    prealpha_001_2_to_prealpha_001_5::migrate as migrate_prealpha_001_2_to_prealpha_001_5,
    prealpha_001_3_to_prealpha_001_5::migrate as migrate_prealpha_001_3_to_prealpha_001_5,
    prealpha_001_4_to_prealpha_001_5::migrate as migrate_prealpha_001_4_to_prealpha_001_5,
};
use super::{
    AnyWeb3ApiManifest, Web3ApiManifest, Web3ApiManifestFormats, LATEST_WEB3_API_MANIFEST_FORMAT,
};
use std::collections::HashMap;

pub type MigratorMap =
    HashMap<String, fn(manifest: &mut AnyWeb3ApiManifest) -> Result<Web3ApiManifest, &'static str>>;
pub struct Migrator;
impl Migrator {
    pub fn generate_migrators() -> MigratorMap {
        let mut migrators: MigratorMap = HashMap::new();
        let _ = migrators.insert(
            "0.0.1-prealpha.1".to_string(),
            migrate_prealpha_001_1_to_prealpha_001_5,
        );
        let _ = migrators.insert(
            "0.0.1-prealpha.2".to_string(),
            migrate_prealpha_001_2_to_prealpha_001_5,
        );
        let _ = migrators.insert(
            "0.0.1-prealpha.3".to_string(),
            migrate_prealpha_001_3_to_prealpha_001_5,
        );
        let _ = migrators.insert(
            "0.0.1-prealpha.4".to_string(),
            migrate_prealpha_001_4_to_prealpha_001_5,
        );
        migrators
    }
}

pub fn migrate_web3_api_manifest(
    manifest: &mut AnyWeb3ApiManifest,
    to: &str,
) -> Result<Web3ApiManifest, String> {
    let from = AnyWeb3ApiManifest::get_manifest_format(manifest);
    let to = Web3ApiManifestFormats::get_format_name(to).unwrap();

    if from.as_str() == LATEST_WEB3_API_MANIFEST_FORMAT {
        match manifest {
            AnyWeb3ApiManifest::Web3ApiManifest001Prealpha5(five) => {
                return Ok(Web3ApiManifest {
                    format: five.format.clone(),
                    build: five.build.clone(),
                    meta: five.meta.clone(),
                    language: five.language.clone(),
                    modules: five.modules.clone(),
                    import_redirects: five.import_redirects.clone(),
                    __type: five.__type.clone(),
                });
            }
            _ => {
                return Err("Unrecognized manifest".to_string());
            }
        }
    }

    if Web3ApiManifestFormats::get_format_name(from.as_str()).is_none() {
        return Err(format!(
            "Unrecognized Web3ApiManifestFormat {}",
            from.as_str()
        ));
    }

    let migrators = Migrator::generate_migrators();
    let migrator = migrators.get(from.as_str());
    if migrator.is_none() {
        return Err(format!(
            "Migrator from Web3ApiManifestFormat {} to {} is not available",
            &from, to
        ));
    }

    Ok(migrator.unwrap()(manifest).unwrap())
}
