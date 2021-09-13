use crate::types::interface_implementations::{
    sanitize_interface_implementations, InterfaceImplementations, InterfaceImplementationsArgs,
};
use crate::types::uri::Uri;

#[test]
fn returns_empty_array_if_empty_array_passed() {
    let mut input: Vec<InterfaceImplementationsArgs> = vec![];
    let sanitized_interface_implementations =
        sanitize_interface_implementations(&mut input).unwrap();
    assert!(sanitized_interface_implementations.is_empty());
}

#[test]
fn returns_interfaces_from_interface_definitions() {
    let input_args = InterfaceImplementationsArgs {
        interface: "w3://w3/interface".to_string(),
        implementations: vec!["w3://w3/api1".to_string(), "w3://w3/api2".to_string()],
    };
    let input: Vec<InterfaceImplementationsArgs> = vec![input_args];
    let sanitized_interface_implementations = sanitize_interface_implementations(&input).unwrap();
    assert_eq!(
        sanitized_interface_implementations,
        vec![InterfaceImplementations {
            interface: Uri::new("w3://w3/interface"),
            implementations: vec![Uri::new("w3://w3/api1"), Uri::new("w3://w3/api2")],
        }]
    );
}
