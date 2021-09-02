use std::collections::HashMap;

use crate::types::{QueryApiInvocations, QueryDocument, Uri};
use graphql_parser::query::Text;

pub fn parse_query<'a, T>(
    _uri: &Uri,
    doc: QueryDocument<'a, T>,
    _variables: Option<HashMap<String, T>>,
) -> Result<QueryApiInvocations, &'a str>
where
    T: Text<'a> + PartialEq,
{
    if doc.definitions.is_empty() {
        return Err("Empty query document found");
    }

    let query_invocations = QueryApiInvocations {};

    for _def in &doc.definitions {}

    Ok(query_invocations)
}
