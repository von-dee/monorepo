use super::{
    AnyPluginManifest, PluginManifest, PluginManifestFormats, LATEST_PLUGIN_MANIFEST_FORMAT,
};
use std::collections::HashMap;

type MigratorMap =
    HashMap<String, fn(manifest: &mut AnyPluginManifest) -> Result<PluginManifest, &'static str>>;
struct Migrator;
impl Migrator {
    pub fn generate_migrators() -> MigratorMap {
        HashMap::new()
    }
}

pub fn migrate_plugin_manifest(
    manifest: &mut AnyPluginManifest,
    to: &str,
) -> Result<PluginManifest, String> {
    let from = AnyPluginManifest::get_manifest_format(manifest);
    let to = PluginManifestFormats::get_format_name(to).unwrap();

    if from.as_str() == LATEST_PLUGIN_MANIFEST_FORMAT {
        match manifest {
            AnyPluginManifest::PluginManifest001Prealpha1(one) => {
                return Ok(PluginManifest {
                    format: one.format.clone(),
                    language: one.language.clone(),
                    schema: one.schema.clone(),
                    import_redirects: one.import_redirects.clone(),
                    __type: one.__type.clone(),
                });
            }
        }
    }

    if PluginManifestFormats::get_format_name(from.as_str()).is_none() {
        return Err(format!(
            "Unrecognized PluginManifestFormat {}",
            from.as_str()
        ));
    }

    let migrators = Migrator::generate_migrators();
    let migrator = migrators.get(from.as_str());
    if migrator.is_none() {
        return Err(format!(
            "This should never happen, PluginManifest migrators is empty. from: {}, to: {}",
            &from, to
        ));
    }

    Ok(migrator.unwrap()(manifest).unwrap())
}
