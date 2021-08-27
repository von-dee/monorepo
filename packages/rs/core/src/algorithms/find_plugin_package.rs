use crate::types::{PluginPackage, PluginRegistration, Uri};

pub fn find_plugin_package(uri: Uri, plugins: &[PluginRegistration]) -> Option<PluginPackage> {
    let plugin_redirect = plugins.iter().find(|x| Uri::equals(&x.uri, &uri));
    Some(plugin_redirect.unwrap().plugin.clone())
}
