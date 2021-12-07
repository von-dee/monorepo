use super::super::prealpha_001_5::Web3ApiManifest as NewManifest;
use super::super::AnyWeb3ApiManifest;

pub fn migrate(any_manifest: &mut AnyWeb3ApiManifest) -> Result<NewManifest, &'static str> {
    match any_manifest {
        AnyWeb3ApiManifest::Web3ApiManifest001Prealpha2(old) => {
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
        _ => Err("Wrong Web3ApiManifest given"),
    }
}
