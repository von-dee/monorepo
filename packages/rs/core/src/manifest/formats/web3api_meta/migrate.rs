use super::{AnyMetaManifest, MetaManifest, MetaManifestFormats, LATEST_META_MANIFEST_FORMAT};
use std::collections::HashMap;

type MigratorMap =
    HashMap<String, fn(manifest: &mut AnyMetaManifest) -> Result<MetaManifest, &'static str>>;
struct Migrator;
impl Migrator {
    pub fn generate_migrators() -> MigratorMap {
        HashMap::new()
    }
}

pub fn migrate_meta_manifest(
    manifest: &mut AnyMetaManifest,
    to: &str,
) -> Result<MetaManifest, String> {
    let from = AnyMetaManifest::get_manifest_format(manifest);
    let to = MetaManifestFormats::get_format_name(to).unwrap();

    if from.as_str() == LATEST_META_MANIFEST_FORMAT {
        match manifest {
            AnyMetaManifest::MetaManifest001Prealpha1(one) => {
                return Ok(MetaManifest {
                    format: one.format.clone(),
                    name: one.name.clone(),
                    subtext: one.subtext.clone(),
                    description: one.description.clone(),
                    repository: one.repository.clone(),
                    icon: one.icon.clone(),
                    links: one.links.clone(),
                    queries: one.queries.clone(),
                    __type: one.__type.clone(),
                });
            }
        }
    }

    if MetaManifestFormats::get_format_name(from.as_str()).is_none() {
        return Err(format!("Unrecognized MetaManifestFormat {}", from.as_str()));
    }

    let migrators = Migrator::generate_migrators();
    let migrator = migrators.get(from.as_str());
    if migrator.is_none() {
        return Err(format!(
            "This should never happen, MetaManifest migrators is empty. {} to {}",
            &from, to
        ));
    }

    Ok(migrator.unwrap()(manifest).unwrap())
}
