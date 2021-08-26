use super::{PluginPackage, Uri};

#[derive(Clone)]
pub struct PluginRegistration {
    uri: Uri,
    plugin: PluginPackage,
}

pub fn sanitize_plugin_registrations(
    input: &[PluginRegistration],
) -> Result<Vec<PluginRegistration>, &str> {
    let mut output: Vec<PluginRegistration> = vec![];
    for definition in input {
        let uri = Uri::new(&definition.uri.get_uri());
        let plugin = definition.plugin.clone();
        output.push(PluginRegistration { uri, plugin });
    }
    Ok(output)
}
