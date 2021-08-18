use super::{Client, InvokeApiOptions, InvokeApiResult};
use std::collections::HashMap;

/// Cache of API definitions, mapping the API's URI to its definition
pub type ApiCache = HashMap<String, Api>;

/// The API definition, which can be used to spawn
/// many invocations of this particular API. Internally
/// this trait may do things like caching WASM bytecode, spawning
/// worker threads, or indexing into resolvers to find the requested method.
pub struct Api {}

impl Api {
    /// Invoke the API based on the provided [[InvokeApiOptions]]
    /// @param options: Options for this invocation.
    /// @param client: The client instance requesting this invocation.
    /// This client will be used for any sub-queries that occur.
    /// TODO: Use Futures
    pub async fn invoke<T>(
        _options: InvokeApiOptions<'_, T>,
        _client: &dyn Client,
    ) -> InvokeApiResult<T> {
        unimplemented!()
    }

    /// Get the API's schema
    /// TODO: Use Futures
    pub async fn get_schema(_client: &dyn Client) -> String {
        unimplemented!()
    }
}
