//! URI type

// use tracing::Tracer when it's implemented

/// URI configuration
#[derive(Clone, Debug, Default, PartialEq)]
pub struct UriConfig {
    pub authority: Option<String>,
    pub path: Option<String>,
    pub uri: Option<String>,
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
#[derive(Clone, Debug, Default, PartialEq)]
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
        self.config.authority.as_ref().unwrap().to_string()
    }

    pub fn get_path(&self) -> String {
        self.config.path.as_ref().unwrap().to_string()
    }

    pub fn get_uri(&self) -> String {
        self.config.uri.as_ref().unwrap().to_string()
    }

    pub fn equals(a: Self, b: Self) -> bool {
        a.config.uri == b.config.uri
    }

    pub fn is_uri<T>(_value: T) -> bool {
        todo!()
    }

    pub fn is_valid_uri(uri: &str, parsed: Option<UriConfig>) -> bool {
        if let Ok(result) = Uri::parse_uri(uri) {
            parsed.unwrap() == result
        } else {
            false
        }
    }

    pub fn parse_uri(_uri: &str) -> Result<UriConfig, &str> {
        todo!()
    }
}
