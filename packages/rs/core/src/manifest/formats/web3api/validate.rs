// use std::collections::HashMap;
// use std::fs;

// use super::{AnyWeb3ApiManifest, Web3ApiManifestFormats};
// use super::super::super::validators;
// use valico::json_schema::{Schema, validators::{Validator}};

// type Web3ApiManifestSchemas = HashMap<&'static str, Option<Schema>>;

// struct SchemaMap;

// impl SchemaMap {
//     pub fn generate_schemas() -> Self {
//         let schema_0_0_1_prealpha_1_path = "../../../../../../packages/manifest-schemas/formats/web3api/0.0.1-prealpha.1.json";
//         let schema_0_0_1_prealpha_2_path = "../../../../../../packages/manifest-schemas/formats/web3api/0.0.1-prealpha.2.json";
//         let schema_0_0_1_prealpha_3_path = "../../../../../../packages/manifest-schemas/formats/web3api/0.0.1-prealpha.3.json";
//         let schema_0_0_1_prealpha_4_path = "../../../../../../packages/manifest-schemas/formats/web3api/0.0.1-prealpha.4.json";
//         let schema_0_0_1_prealpha_5_path = "../../../../../../packages/manifest-schemas/formats/web3api/0.0.1-prealpha.5.json";

//         let schema_0_0_1_prealpha_1 = fs::read_to_string(schema_0_0_1_prealpha_1_path).unwrap();
//         let schema_0_0_1_prealpha_2 = fs::read_to_string(schema_0_0_1_prealpha_2_path).unwrap();
//         let schema_0_0_1_prealpha_3 = fs::read_to_string(schema_0_0_1_prealpha_3_path).unwrap();
//         let schema_0_0_1_prealpha_4 = fs::read_to_string(schema_0_0_1_prealpha_4_path).unwrap();
//         let schema_0_0_1_prealpha_5 = fs::read_to_string(schema_0_0_1_prealpha_5_path).unwrap();

//         let mut schemas: Web3ApiManifestSchemas = HashMap::new();
//         todo!()
//     }
// }
