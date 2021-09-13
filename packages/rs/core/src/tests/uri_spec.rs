use crate::types::uri::Uri;

#[test]
fn inserts_a_w3_scheme_when_one_is_not_present() {
    let uri = Uri::new("/authority-v2/path.to.thing.root/sub/path");
    assert_eq!(
        &uri.get_uri().unwrap(),
        "w3://authority-v2/path.to.thing.root/sub/path"
    );
    assert_eq!(&uri.get_authority().unwrap(), "authority-v2");
    assert_eq!(&uri.get_path().unwrap(), "path.to.thing.root/sub/path");
}

#[test]
fn is_uri_returns_false_when_given_something_not_a_uri() {
    assert!(!Uri::is_uri("/authority-v2/path.to.thing.root/sub/path"));
}

#[test]
#[should_panic]
fn panics_if_an_authority_is_not_present() {
    Uri::new("w3://path").get_authority().unwrap();
}

#[test]
#[should_panic]
fn panics_if_a_path_is_not_present() {
    Uri::new("w3://authority/").get_path().unwrap();
}

#[test]
#[should_panic]
fn panics_if_scheme_is_not_at_the_beginning() {
    Uri::new("path/w3://something");
}

#[test]
#[should_panic]
fn panics_if_uri_is_empty() {
    Uri::new("");
}

#[test]
fn returns_true_if_is_valid_uri() {
    assert!(Uri::is_valid_uri("w3://valid/uri"));
}

#[test]
fn returns_false_if_is_invalid_uri() {
    assert!(!Uri::is_valid_uri("w3://....."));
}

#[test]
fn returns_a_parsed_uri_configuration() {
    assert!(Uri::is_valid_uri("w3://valid/uri"));
    let config = Uri::parse_uri("w3://valid/uri").unwrap();
    assert_eq!(config.uri.unwrap(), "w3://valid/uri".to_string());
    assert_eq!(config.authority.unwrap(), "valid".to_string());
    assert_eq!(config.path.unwrap(), "uri".to_string());
}
