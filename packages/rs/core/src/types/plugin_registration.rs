use super::{PluginPackage, Uri};
use polywrap_tracing_rs::Tracer;

#[derive(Clone)]
pub struct PluginRegistration {
    uri: Uri,
    plugin: PluginPackage,
}

pub fn sanitize_plugin_registrations(_tracer: Tracer) {
    // TODO: let input = tracer.trace_func(args: T, span: &'static str, func: fn(args: T) -> Result<T, Error>);
    let input: Vec<PluginRegistration> = vec![];
    let mut output: Vec<PluginRegistration> = vec![];
    for definition in input {
        let uri = Uri::new(&definition.uri.get_uri());
        let plugin = definition.plugin;
        output.push(PluginRegistration { uri, plugin });
    }
}
