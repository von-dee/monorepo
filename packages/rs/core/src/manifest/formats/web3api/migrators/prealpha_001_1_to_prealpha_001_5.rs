use super::super::prealpha_001_5::Web3ApiManifest as NewManifest;
use super::super::AnyWeb3ApiManifest;
use super::super::Modules;

pub fn migrate(any_manifest: &mut AnyWeb3ApiManifest) -> Result<NewManifest, &'static str> {
    match any_manifest {
        AnyWeb3ApiManifest::Web3ApiManifest001Prealpha1(old) => {
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
                import_redirects: old.import_redirects.clone(),
                __type: "Web3ApiManifest".to_string(),
            })
        }
        _ => Err("Wrong Web3ApiManifest given"),
    }
}
