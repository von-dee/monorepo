// TODO

// use super::{InvokeApiOptions, Uri};
use serde::{Deserialize, Serialize};
// use std::collections::HashMap;

/// To parse and execute a given query
pub trait QueryHandler: Clone {}

/// Options required for an API query
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct QueryApiOptions {}

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct QueryApiInvocations {}

pub struct QueryApiResult<T> {
    /// Invoke result data. The type of this value is the return type
    /// of the method.
    pub data: Option<T>,
    /// Errors encountered during invocation
    pub error: Option<Error>,
}

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct Error;
