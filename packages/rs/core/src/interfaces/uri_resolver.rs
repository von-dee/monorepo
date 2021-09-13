use crate::types::{
    Client, InvokableModules, InvokeApiOptions, InvokeApiResult, InvokeHandler, Uri,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct MaybeUriOrManifest {
    pub uri: Option<String>,
    pub manifest: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct Query;

impl Query {
    pub async fn try_resolve_uri(
        mut client: Client,
        api: Uri,
        uri: Uri,
    ) -> InvokeApiResult<MaybeUriOrManifest> {
        let mut input: HashMap<String, Vec<u8>> = HashMap::new();
        input
            .insert(
                uri.get_authority().unwrap(),
                uri.get_path().unwrap().as_bytes().to_vec(),
            )
            .expect("Error in inserting options");
        let options = InvokeApiOptions::<MaybeUriOrManifest> {
            uri: api,
            module: InvokableModules::Query("query".to_string()),
            method: "try_resolve_uri".to_string(),
            input,
            result_filer: None,
            decode: None,
        };
        <Client as InvokeHandler<MaybeUriOrManifest>>::invoke::<MaybeUriOrManifest>(
            &mut client,
            options,
        )
    }

    pub async fn get_file(mut client: Client, api: Uri, path: String) -> InvokeApiResult<Vec<u8>> {
        let mut input: HashMap<String, Vec<u8>> = HashMap::new();
        input
            .insert(path, api.get_path().unwrap().as_bytes().to_vec())
            .expect("Error in inserting options");
        let options = InvokeApiOptions::<Vec<u8>> {
            uri: api,
            module: InvokableModules::Query("query".to_string()),
            method: "get_file".to_string(),
            input,
            result_filer: None,
            decode: None,
        };
        <Client as InvokeHandler<Vec<u8>>>::invoke::<Vec<u8>>(&mut client, options)
    }
}
