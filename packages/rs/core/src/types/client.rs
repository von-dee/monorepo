use super::invoke::{InvokeApiOptions, InvokeApiResult, InvokeHandler};
use super::QueryHandler;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct Client;
impl<T> InvokeHandler<T> for Client {
    fn invoke<F>(&mut self, _options: InvokeApiOptions<F>) -> InvokeApiResult<F> {
        unimplemented!()
    }
}
impl QueryHandler for Client {}
