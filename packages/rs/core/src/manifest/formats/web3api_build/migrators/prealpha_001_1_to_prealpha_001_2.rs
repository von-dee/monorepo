use super::super::prealpha_001_2::BuildManifest as NewManifest;
use super::super::AnyBuildManifest;

pub fn migrate<T: Clone>(
    any_manifest: &mut AnyBuildManifest<T>,
) -> Result<NewManifest<T>, &'static str> {
    match any_manifest {
        AnyBuildManifest::BuildManifest001Prealpha1(old) => Ok(NewManifest {
            format: "0.0.1-prealpha.2".to_string(),
            docker: old.docker.clone(),
            config: old.config.clone(),
            linked_packages: None,
            __type: "BuildManifest".to_string(),
        }),
        _ => Err("Wrong BuildManifest given"),
    }
}
