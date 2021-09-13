use crate::types::plugin::PluginPackage;
use crate::types::plugin_registration::{
    sanitize_plugin_registrations, PluginRegistration, PluginRegistrationArgs,
};
use crate::types::uri::Uri;

#[test]
fn returns_empty_array_if_empty_array_passed() {
    let mut input: Vec<PluginRegistrationArgs> = vec![];
    let sanitized_plugin_registrations = sanitize_plugin_registrations(&mut input).unwrap();
    assert!(sanitized_plugin_registrations.is_empty());
}

#[test]
fn returns_plugins_from_plugin_definitions() {
    let input_args = PluginRegistrationArgs {
        uri: "w3://w3/api".to_string(),
        plugin: PluginPackage::default(),
    };
    let input: Vec<PluginRegistrationArgs> = vec![input_args];
    let sanitized_plugin_registrations = sanitize_plugin_registrations(&input).unwrap();
    assert_eq!(
        sanitized_plugin_registrations,
        vec![PluginRegistration {
            uri: Uri::new("w3://w3/api"),
            plugin: PluginPackage::default(),
        }]
    );
}
