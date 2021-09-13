//! URI type

use regex::Regex;
use serde::{Deserialize, Serialize};
use std::string::ToString;

/// URI configuration
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
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
/// [w3://] - URI Scheme: differentiates Polywrap URIs.
/// [ipfs/] - URI Authority: allows the Polywrap URI resolution algorithm to determine an authoritative URI resolver.
/// [sub.domain.eth] - URI Path: tells the Authority where the API resides.
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct Uri {
    config: UriConfig,
}

impl Uri {
    pub fn new(uri: &str) -> Self {
        Uri {
            config: Uri::parse_uri(uri).expect("Failed to parse URI"),
        }
    }

    pub fn get_authority(&self) -> Option<String> {
        self.config.authority.clone()
    }

    pub fn get_path(&self) -> Option<String> {
        self.config.path.clone()
    }

    pub fn get_uri(&self) -> Option<String> {
        self.config.uri.clone()
    }

    pub fn equals(a: &Self, b: &Self) -> bool {
        a.config.uri == b.config.uri
    }

    pub fn is_uri<T: ToString>(value: T) -> bool {
        let val_str = value.to_string();
        let re = Regex::new(r"/([a-z][a-z0-9-_]+)://([a-z][a-z0-9-_]+)/(.*)/")
            .unwrap()
            .find(&val_str);
        !val_str.is_empty() && re.is_some()
    }

    pub fn is_valid_uri(uri: &str) -> bool {
        let uri_config = Uri::parse_uri(uri).expect("Failed to parse URI");
        uri_config.authority.is_some() && uri_config.path.is_some() && uri_config.uri.is_some()
    }

    pub fn parse_uri(uri: &str) -> Result<UriConfig, String> {
        if uri.as_bytes().is_empty() {
            return Err("The provided URI is empty".to_string());
        }
        let mut processed = uri.to_string();

        // Trim preceding '/' characters
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
        let re = Regex::new(r"/w3://([a-z][a-z0-9-_]+)/(.*)/")
            .unwrap()
            .find(&processed);

        // Remove whitespaces
        if re.is_some() {
            result = re
                .expect("")
                .as_str()
                .chars()
                .filter(|c| !c.is_whitespace())
                .collect()
        } else if re.is_none() {
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

impl std::fmt::Display for UriConfig {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> core::fmt::Result {
        let mut formatted = String::new();
        formatted.push_str(&format!(
            "{{\
			authority: {:#?}, \
			path: {:?}, \
			uri: {}, \
			}}",
            self.authority.as_ref().unwrap(),
            self.path.as_ref().unwrap(),
            self.uri.as_ref().unwrap(),
        ));
        write!(f, "{}", formatted)
    }
}

impl std::fmt::Display for Uri {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> core::fmt::Result {
        let mut formatted = String::new();
        formatted.push_str(&format!(
            "{{\
			config: {:#?}, \
			}}",
            self.config,
        ));
        write!(f, "{}", formatted)
    }
}
