use super::super::prealpha_001_2::Web3ApiManifest as OldManifest;
use super::super::prealpha_001_5::Web3ApiManifest as NewManifest;

pub fn migrate(old: &mut OldManifest) -> Result<NewManifest, &'static str> {
    old.repository = None;
    Ok(NewManifest {
        format: "0.0.1-prealpha.5".to_string(),
        build: old.build.clone(),
        meta: None,
        language: old.language.clone(),
        modules: old.modules.clone(),
        import_redirects: old.import_redirects.clone(),
        __type: "Web3ApiManifest".to_string(),
    })
}
