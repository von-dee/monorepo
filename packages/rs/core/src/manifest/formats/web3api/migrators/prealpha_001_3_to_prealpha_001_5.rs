use super::super::prealpha_001_3::Web3ApiManifest as OldManifest;
use super::super::prealpha_001_5::Web3ApiManifest as NewManifest;

pub fn migrate(old: &mut OldManifest) -> Result<NewManifest, &'static str> {
    old.repository = None;
    let mut language = old.language.clone();

    if language.is_none() {
        if old.interface {
            language = Some("interface".to_string());
        } else {
            language = Some("wasm/rust".to_string());
        }
    }

    Ok(NewManifest {
        format: "0.0.1-prealpha.5".to_string(),
        build: old.build.clone(),
        meta: None,
        language: language.unwrap(),
        modules: old.modules.clone(),
        import_redirects: old.import_redirects.clone(),
        __type: "Web3ApiManifest".to_string(),
    })
}
