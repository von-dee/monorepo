use crate::types::Uri;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct CoreInterfaceUris {
    pub uri_resolver: Uri,
    pub logger: Uri,
}

impl CoreInterfaceUris {
    pub fn new() -> Self {
        let uri_resolver = Uri::new("w3://ens/uri-resolver.core.web3api.eth");
        let logger = Uri::new("w3://ens/logger.core.web3api.eth");
        Self {
            uri_resolver,
            logger,
        }
    }
}
