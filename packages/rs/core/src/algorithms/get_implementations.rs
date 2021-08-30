use super::apply_redirects;
use crate::types::{InterfaceImplementations, Uri, UriRedirect};

pub fn get_implementations(
    api_interface_uri: Uri,
    redirects: &[UriRedirect],
    interfaces: &[InterfaceImplementations],
) -> Vec<Uri> {
    let mut result: Vec<Uri> = vec![];

    let mut add_unique_result = |uri: &Uri| {
        // If the URI hasn't been added already
        if !result.iter().any(|i| Uri::equals(i, uri)) {
            result.push(uri.clone());
        }
    };
    let mut add_all_implementations_from_implementations_array =
        |implementations_array: &[InterfaceImplementations], api_interface_uri: Uri| {
            for interface_implementations in implementations_array {
                let fully_resolved_uri = apply_redirects::apply_redirects(
                    &interface_implementations.interface,
                    redirects,
                )
                .expect("Failed to apply redirects");
                if Uri::equals(&fully_resolved_uri, &api_interface_uri) {
                    for implementation in &interface_implementations.implementations {
                        add_unique_result(implementation);
                    }
                }
            }
        };
    let final_redirected_api_interface =
        apply_redirects::apply_redirects(&api_interface_uri, redirects)
            .expect("Failed to apply redirects");
    add_all_implementations_from_implementations_array(interfaces, final_redirected_api_interface);

    result
}
