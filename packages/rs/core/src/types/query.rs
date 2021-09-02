use super::{InvokeApiOptions, Uri};
use graphql_parser::{
    parse_query,
    query::{Document, Text},
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// To parse and execute a given query
pub trait QueryHandler<'a, T>: Clone {
    fn query<F: Text<'a>>(&mut self, _options: QueryApiOptions<'a, F>) -> QueryApiResult<F>;
}

/// GraphQL QueryDocument
pub type QueryDocument<'a, T> = Document<'a, T>;

/// Create a GraphQL QueryDocument by parsing a string
pub fn create_query_document<'a, T: Text<'a>>(query: &'a str) -> QueryDocument<'a, T> {
    parse_query(query).expect("Failed to parse query")
}

/// Options required for an API query
#[derive(Clone, Debug, PartialEq)]
pub struct QueryApiOptions<'a, T: Text<'a>> {
    /// The API's URI
    pub uri: Uri,
    /// The GraphQL query to parse and execute, leading to one or more
    /// API invocations.
    pub query: QueryDocument<'a, T>,
    /// Variables referenced within the query string via GraphQL's `$variable` syntax
    pub variables: Option<HashMap<String, T>>,
}

/// The result of an API query, which is the aggregate
/// of one or more [[InvokeApiResult | invocation results]]
pub struct QueryApiResult<T> {
    /// Query result data. The type of this value is a named map,
    /// where the key is the method's name, and value is the [[InvokeApiResult]]'s data.
    /// This is done to allow for parallel invocations within a
    /// single query document. In case of method name collisions,
    /// a postfix of `_0` will be applied, where 0 will be incremented for
    /// each occurrence. If undefined, it means something went wrong.
    /// Errors should be populated with information as to what happened.
    /// None is used to represent an intentionally null result.
    pub data: Option<T>,
    /// Errors encountered during query
    pub errors: Option<Vec<String>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct QueryApiInvocations<T> {
    pub method_or_alias: InvokeApiOptions<T>,
}
