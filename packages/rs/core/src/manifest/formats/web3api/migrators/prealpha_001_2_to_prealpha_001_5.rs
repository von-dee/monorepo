use super::super::prealpha_001_2::Web3ApiManifest as OldManifest;
use super::super::prealpha_001_5::Web3ApiManifest as NewManifest;
use super::super::Modules;

pub fn migrate(old: &mut OldManifest) -> Result<NewManifest, &'static str> {
    old.repository = None;
    Ok(NewManifest {
        format: "0.0.1-prealpha.5".to_string(),
        build: None,
        meta: None,
        language: old.language.clone(),
        modules: Modules {
            mutation: old.modules.mutation.clone(),
            query: old.modules.query.clone(),
        },
        import_redirects: None,
        __type: "Web3ApiManifest".to_string(),
    })
}
