use super::{error::Error, uri::Uri};
use std::collections::HashMap;

//pub type InvokableModules<'a> = Option<&'a str>; ???

#[derive(Debug, Clone)]
pub enum InvokableModules {
    Mutation(String),
    Query(String),
}

/// Options required for an API invocation
#[derive(Debug, Clone)]
pub struct InvokeApiOptions<T> {
    /// The API's URI
    uri: Uri,
    /// Module to be called into
    module: InvokableModules,
    /// Method to be executed
    method: String,
    /// Input arguments for the method, structured as a map,
    /// removing the chance of incorrectly ordering arguments.
    input: HashMap<String, Vec<u8>>,
    /// Filters the [[InvokeApiResult]] data properties. The key
    /// of this map is the property's name, while the value is
    /// either true (meaning select this prop), or a nested named map,
    /// allowing for the filtering of nested objects.
    result_filer: HashMap<String, Option<T>>,
    /// If set to true, the invoke function will decode all msgpack results
    /// into objects
    decode: Option<bool>,
}

/// Result of an API invokation
#[derive(Debug, Clone)]
pub struct InvokeApiResult<T> {
    /// Invoke result data. The type of this value is the return type
    /// of the method.
    data: Option<T>,
    /// Errors encountered during invocation
    error: Option<Error>,
}

// TODO: Use Futures
pub trait InvokeHandler<T> {
    fn invoke_str<F>(options: InvokeApiOptions<&str>) -> InvokeApiResult<F>;
    fn invoke_uri<F>(options: InvokeApiOptions<Uri>) -> InvokeApiResult<F>;
}
