use super::invoke::{InvokeApiOptions, InvokeApiResult, InvokeHandler};
use super::query::{QueryApiOptions, QueryApiResult, QueryHandler};
use graphql_parser::query::Text;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct Client;
impl<T> InvokeHandler<T> for Client {
    fn invoke<F>(&mut self, _options: InvokeApiOptions<F>) -> InvokeApiResult<F> {
        unimplemented!()
    }
}
impl<'a, T> QueryHandler<'a, T> for Client {
    fn query<F: Text<'a>>(&mut self, _options: QueryApiOptions<'a, F>) -> QueryApiResult<F> {
        unimplemented!()
    }
}
