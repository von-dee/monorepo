//! URI type

// use tracing::Tracer when it's implemented

/// URI configuration
#[derive(Debug, Clone, Default)]
pub struct UriConfig {
    authority: String,
    path: String,
    uri: String,
}

/// A Polywrap URI. Some examples of valid URIs are:
/// w3://ipfs/QmHASH
/// w3://ens/sub.dimain.eth
/// w3://fs/directory/file.txt
/// w3://uns/domain.crypto
/// Breaking down the various parts of the URI, as it applies
/// to [the URI standard](https://tools.ietf.org/html/rfc3986#section-3):
/// **w3://** - URI Scheme: differentiates Polywrap URIs.
/// **ipfs/** - URI Authority: allows the Polywrap URI resolution algorithm to determine an authoritative URI resolver.
/// **sub.domain.eth** - URI Path: tells the Authority where the API resides.
#[derive(Debug, Clone, Default)]
pub struct Uri {
    config: UriConfig,
}

impl Uri {
    pub fn new(uri: &str) -> Self {
        Uri {
            config: Uri::parse_uri(uri).expect("Failed to parse URI"),
        }
    }

    pub fn get_authority(&self) -> String {
        self.config.authority.clone()
    }

    pub fn get_path(&self) -> String {
        self.config.path.clone()
    }

    pub fn get_uri(&self) -> String {
        self.config.uri.clone()
    }

    pub fn equals(a: Self, b: Self) -> bool {
        a.get_uri() == b.get_uri()
    }

    pub fn is_uri<T>(_value: T) -> bool {
        todo!()
    }

    pub fn is_valid_uri(uri: &str, parsed: Option<UriConfig>) -> bool {
        //matches!(Uri::parse_uri(uri), Ok(_))
        if Uri::parse_uri(uri).is_ok() {
            if parsed.is_some() {
                // What is: parse = Object.assign(parsed, result) ?
            }
            true
        } else {
            false
        }
    }

    pub fn parse_uri(_uri: &str) -> Result<UriConfig, &str> {
        todo!()
    }
}
