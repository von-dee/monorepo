use super::super::prealpha_001_1::Web3ApiManifest as OldManifest;
use super::super::prealpha_001_5::Web3ApiManifest as NewManifest;
use super::super::Modules;

pub fn migrate(old: &mut OldManifest) -> Result<NewManifest, &'static str> {
    old.repository = None;
    let module = old.module.as_ref();

    if module.is_none() {
        return Err("No module found");
    }

    Ok(NewManifest {
        format: "0.0.1-prealpha.5".to_string(),
        build: None,
        meta: None,
        language: module.unwrap().language.clone(),
        modules: Modules {
            mutation: old.mutation.clone(),
            query: old.query.clone(),
        },
        import_redirects: None,
        __type: "Web3ApiManifest".to_string(),
    })
}
