use std::collections::HashMap;
use std::fs;

use super::AnyWeb3ApiManifest;
use valico::json_schema;

type Web3ApiManifestSchemas = HashMap<&'static str, serde_json::Value>;

fn generate_schemas() -> Web3ApiManifestSchemas {
    let schema_0_0_1_prealpha_1_path =
        "../../../../../../packages/manifest-schemas/formats/web3api/0.0.1-prealpha.1.json";
    let schema_0_0_1_prealpha_2_path =
        "../../../../../../packages/manifest-schemas/formats/web3api/0.0.1-prealpha.2.json";
    let schema_0_0_1_prealpha_3_path =
        "../../../../../../packages/manifest-schemas/formats/web3api/0.0.1-prealpha.3.json";
    let schema_0_0_1_prealpha_4_path =
        "../../../../../../packages/manifest-schemas/formats/web3api/0.0.1-prealpha.4.json";
    let schema_0_0_1_prealpha_5_path =
        "../../../../../../packages/manifest-schemas/formats/web3api/0.0.1-prealpha.5.json";

    let schema_0_0_1_prealpha_1: serde_json::Value =
        serde_json::from_str(&fs::read_to_string(schema_0_0_1_prealpha_1_path).unwrap()).unwrap();
    let schema_0_0_1_prealpha_2: serde_json::Value =
        serde_json::from_str(&fs::read_to_string(schema_0_0_1_prealpha_2_path).unwrap()).unwrap();
    let schema_0_0_1_prealpha_3: serde_json::Value =
        serde_json::from_str(&fs::read_to_string(schema_0_0_1_prealpha_3_path).unwrap()).unwrap();
    let schema_0_0_1_prealpha_4: serde_json::Value =
        serde_json::from_str(&fs::read_to_string(schema_0_0_1_prealpha_4_path).unwrap()).unwrap();
    let schema_0_0_1_prealpha_5: serde_json::Value =
        serde_json::from_str(&fs::read_to_string(schema_0_0_1_prealpha_5_path).unwrap()).unwrap();

    let mut schemas: Web3ApiManifestSchemas = HashMap::new();

    let _ = schemas.insert("0.0.1-prealpha.1", schema_0_0_1_prealpha_1);
    let _ = schemas.insert("0.0.1-prealpha.2", schema_0_0_1_prealpha_2);
    let _ = schemas.insert("0.0.1-prealpha.3", schema_0_0_1_prealpha_3);
    let _ = schemas.insert("0.0.1-prealpha.4", schema_0_0_1_prealpha_4);
    let _ = schemas.insert("0.0.1-prealpha.5", schema_0_0_1_prealpha_5);
    schemas
}

pub fn validate_web3_api_manifest(
    manifest: &AnyWeb3ApiManifest,
    ext_schema: Option<serde_json::Value>,
) -> Result<(), String> {
    let manifest_format = AnyWeb3ApiManifest::get_manifest_format(manifest);
    let schemas = generate_schemas();
    let schema = schemas.get(manifest_format.as_str());
    if schema.is_none() {
        return Err(format!(
            "Unrecognized Web3ApiManifest schema format {}\nmanifest: {:#?}",
            manifest_format, manifest
        ));
    }

    let mut scope = json_schema::Scope::new();
    let manifest_schema = match manifest {
        AnyWeb3ApiManifest::Web3ApiManifest001Prealpha1(one) => {
            serde_json::json!(one)
        }
        AnyWeb3ApiManifest::Web3ApiManifest001Prealpha2(two) => {
            serde_json::json!(two)
        }
        AnyWeb3ApiManifest::Web3ApiManifest001Prealpha3(three) => {
            serde_json::json!(three)
        }
        AnyWeb3ApiManifest::Web3ApiManifest001Prealpha4(four) => {
            serde_json::json!(four)
        }
        AnyWeb3ApiManifest::Web3ApiManifest001Prealpha5(five) => {
            serde_json::json!(five)
        }
    };
    let scoped_schema = scope
        .compile_and_return(schema.unwrap().to_owned(), true)
        .ok()
        .expect("Validation errors encountered while sanitizing Web3ApiManifest format");
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
