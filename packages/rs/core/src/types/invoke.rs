use super::{error::Error, uri::Uri};
use std::collections::HashMap;

pub trait InvokeHandler<T>: Clone {
    // fn invoke_str<F>(options: InvokeApiOptions<&str>) -> InvokeApiResult<F> {
    //     unimplemented!()
    // }
    fn invoke<F>(&mut self, _options: InvokeApiOptions<F>) -> InvokeApiResult<F>;
}

#[derive(Debug, Clone)]
pub enum InvokableModules {
    Mutation(String),
    Query(String),
}

/// Options required for an API invocation
#[derive(Debug, Clone)]
pub struct InvokeApiOptions<T> {
    /// The API's URI
    pub uri: Uri,
    /// Module to be called into
    pub module: InvokableModules,
    /// Method to be executed
    pub method: String,
    /// Input arguments for the method, structured as a map,
    /// removing the chance of incorrectly ordering arguments.
    pub input: HashMap<String, Vec<u8>>,
    /// Filters the [[InvokeApiResult]] data properties. The key
    /// of this map is the property's name, while the value is
    /// either true (meaning select this prop), or a nested named map,
    /// allowing for the filtering of nested objects.
    pub result_filer: Option<HashMap<String, T>>,
    /// If set to true, the invoke function will decode all msgpack results
    /// into objects
    pub decode: Option<bool>,
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
