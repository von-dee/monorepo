use std::collections::HashMap;
use std::fs;

use super::AnyBuildManifest;
use valico::json_schema;
type BuildManifestSchemas = HashMap<&'static str, serde_json::Value>;

fn generate_schemas() -> BuildManifestSchemas {
    let schema_0_0_1_prealpha_1_path =
        "../../../../../../packages/manifest-schemas/formats/web3api.build/0.0.1-prealpha.1.json";
    let schema_0_0_1_prealpha_2_path =
        "../../../../../../packages/manifest-schemas/formats/web3api.build/0.0.1-prealpha.2.json";

    let schema_0_0_1_prealpha_1: serde_json::Value =
        serde_json::from_str(&fs::read_to_string(schema_0_0_1_prealpha_1_path).unwrap()).unwrap();
    let schema_0_0_1_prealpha_2: serde_json::Value =
        serde_json::from_str(&fs::read_to_string(schema_0_0_1_prealpha_2_path).unwrap()).unwrap();

    let mut schemas: BuildManifestSchemas = HashMap::new();

    let _ = schemas.insert("0.0.1-prealpha.1", schema_0_0_1_prealpha_1);
    let _ = schemas.insert("0.0.1-prealpha.2", schema_0_0_1_prealpha_2);
    schemas
}

pub fn validate_build_manifest(
    manifest: &AnyBuildManifest,
    ext_schema: Option<serde_json::Value>,
) -> Result<(), String> {
    let manifest_format = AnyBuildManifest::get_manifest_format(manifest);
    let schemas = generate_schemas();
    let schema = schemas.get(manifest_format.as_str());
    if schema.is_none() {
        return Err(format!(
            "Unrecognized BuildManifest schema format {}\nmanifest: {:#?}",
            manifest_format, manifest
        ));
    }

    let mut scope = json_schema::Scope::new();
    let manifest_schema = match manifest {
        AnyBuildManifest::BuildManifest001Prealpha1(one) => {
            serde_json::json!(one)
        }
        AnyBuildManifest::BuildManifest001Prealpha2(two) => {
            serde_json::json!(two)
        }
    };
    let scoped_schema = scope
        .compile_and_return(schema.unwrap().to_owned(), true)
        .ok()
        .expect("Validation errors encountered while sanitizing BuildManifest format");
    if !scoped_schema.validate(&manifest_schema.clone()).is_valid() {
        return Err("Got an invalid schema".to_string());
    }

    if ext_schema.is_some() {
        let scoped_schema = scope
            .compile_and_return(ext_schema.unwrap().to_owned(), true)
            .ok()
            .expect("Validation errors encountered while sanitizing ext schema");
        if !scoped_schema.validate(&manifest_schema).is_valid() {
            return Err("Invalid ext schema".to_string());
        }
    }

    Ok(())
}
