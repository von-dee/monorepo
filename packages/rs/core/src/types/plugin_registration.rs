use super::{PluginPackage, Uri};

#[derive(Clone, Default, PartialEq, Debug)]
pub struct PluginRegistration {
    pub uri: Uri,
    pub plugin: PluginPackage,
}

#[derive(Clone, Debug)]
pub struct PluginRegistrationArgs {
    pub uri: String,
    pub plugin: PluginPackage,
}

pub fn sanitize_plugin_registrations(
    input: &[PluginRegistrationArgs],
) -> Result<Vec<PluginRegistration>, &str> {
    let mut output: Vec<PluginRegistration> = vec![];
    for definition in input {
        let uri = Uri::new(&definition.uri);
        let plugin = definition.plugin.clone();
        output.push(PluginRegistration { uri, plugin });
    }
    Ok(output)
}
