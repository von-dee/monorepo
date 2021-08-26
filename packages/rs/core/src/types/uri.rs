//! URI type

// use tracing::Tracer when it's implemented
use regex::Regex;

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

    pub fn parse_uri(uri: &str) -> Result<UriConfig, String> {
        if uri.as_bytes().is_empty() {
            return Err("The provided URI is empty".to_string());
        }
        let mut processed = uri.to_string();

        // Trim preceding '/' characters
        // Why are we using a `while` loop here?
        while processed.starts_with('/') {
            processed = processed.chars().nth(1).unwrap().to_string();
        }
        // Check for the w3:// scheme, add if it isn't there
        // let w3_scheme_idx = processed.find("w3://").unwrap();
        // If it's missing the w3:// scheme, add it
        if !processed.contains("w3://") {
            processed = format!("{}{}", "w3://", processed);
        }

        // If the w3:// is not in the beginning, throw an error
        if !processed.starts_with("w3://") {
            return Err("The w3:// scheme must be at the beginning of the URI string".to_string());
        }

        // Extract the authority & path
        let mut result = String::from("");
        let re = Regex::new(r"w3://([a-z][a-z0-9-_]+)/(.*)")
            .unwrap()
            .find(&processed);

        // Remove whitespaces
        if let Ok(re) = Regex::new(r"w3://([a-z][a-z0-9-_]+)/(.*)") {
            result = re.as_str().chars().filter(|c| !c.is_whitespace()).collect()
        } else if re.is_none() || re.unwrap().as_str().len() != 3 {
            let uri_received = format!("Invalid URI Received: {}", uri);
            let error = format!(
                "{}\n{}\n{}\n{}\n\n{}",
                "URI is malformed, here are some examples of valid URIs:",
                "w3://ipfs/QmHASH",
                "w3://ens/domain.eth",
                "ens/domain.eth",
                uri_received
            );
            return Err(error);
        }
        Ok(UriConfig {
            uri: Some(processed.to_string()),
            //authority: result[1],
            //path: result[2],
            // ^FIXME
            authority: Some(result.clone()),
            path: Some(result),
        })
    }
}
