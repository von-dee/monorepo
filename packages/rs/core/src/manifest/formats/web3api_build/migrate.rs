use serde::Serialize;

use super::migrators::prealpha_001_1_to_prealpha_001_2::migrate as migrate_prealpha_001_1_to_prealpha_001_2;
use super::{AnyBuildManifest, BuildManifest, BuildManifestFormats, LATEST_BUILD_MANIFEST_FORMAT};
use std::collections::HashMap;

type MigratorMap<T> = HashMap<
    String,
    fn(manifest: &mut AnyBuildManifest<T>) -> Result<BuildManifest<T>, &'static str>,
>;
fn generate_migrators<T: Clone + std::fmt::Debug + Serialize>() -> MigratorMap<T> {
    let mut migrators: MigratorMap<T> = HashMap::new();
    let _ = migrators.insert(
        "0.0.1-prealpha.1".to_string(),
        migrate_prealpha_001_1_to_prealpha_001_2,
    );
    migrators
}

pub fn migrate_build_manifest<T: Clone + std::fmt::Debug + Serialize>(
    manifest: &mut AnyBuildManifest<T>,
    to: &str,
) -> Result<BuildManifest<T>, String> {
    let from = AnyBuildManifest::get_manifest_format(manifest);
    let to = BuildManifestFormats::get_format_name(to).unwrap();

    if from.as_str() == LATEST_BUILD_MANIFEST_FORMAT {
        match manifest {
            AnyBuildManifest::BuildManifest001Prealpha2(two) => {
                return Ok(BuildManifest {
                    format: two.format.clone(),
                    docker: two.docker.clone(),
                    config: two.config.clone(),
                    linked_packages: two.linked_packages.clone(),
                    __type: two.__type.clone(),
                });
            }
            _ => {
                return Err("Unrecognized manifest".to_string());
            }
        }
    }

    if BuildManifestFormats::get_format_name(from.as_str()).is_none() {
        return Err(format!(
            "Unrecognized BuildManifestFormat {}",
            from.as_str()
        ));
    }

    let migrators = generate_migrators();
    let migrator = migrators.get(from.as_str());
    if migrator.is_none() {
        return Err(format!(
            "Migrator from BuildManifestFormat {} to {} is not available",
            &from, to
        ));
    }

    Ok(migrator.unwrap()(manifest).unwrap())
}
