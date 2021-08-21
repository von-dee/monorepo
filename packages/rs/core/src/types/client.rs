use super::{InvokeHandler, QueryHandler};

#[derive(Clone)]
pub struct Client;
impl InvokeHandler for Client {}
impl QueryHandler for Client {}
