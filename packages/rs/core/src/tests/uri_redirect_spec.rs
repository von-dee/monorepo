use crate::types::uri::Uri;
use crate::types::uri_redirect::{sanitize_uri_redirects, UriRedirect, UriRedirectArgs};

#[test]
fn returns_empty_array_if_empty_array_passed() {
    let mut input: Vec<UriRedirectArgs> = vec![];
    let sanitized_uri_redirects = sanitize_uri_redirects(&mut input).unwrap();
    assert!(sanitized_uri_redirects.is_empty());
}

#[test]
fn returns_interfaces_from_interface_definitions() {
    let input_args = UriRedirectArgs {
        from: "w3://w3/api".to_string(),
        to: "w3://w3/api".to_string(),
    };
    let input: Vec<UriRedirectArgs> = vec![input_args];
    let sanitized_uri_redirects = sanitize_uri_redirects(&input).unwrap();
    assert_eq!(
        sanitized_uri_redirects,
        vec![UriRedirect {
            from: Some(Uri::new("w3://w3/api")),
            to: Some(Uri::new("w3://w3/api")),
        }]
    );
}
