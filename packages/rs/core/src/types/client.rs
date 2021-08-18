use super::{InvokeHandler, QueryHandler};

pub trait Client: InvokeHandler + QueryHandler {}
