use super::Uri;
use polywrap_tracing_rs::Tracer;

#[derive(Clone, Debug)]
pub struct InterfaceImplementations {
    interface: Uri,
    implementations: Vec<Uri>,
}

pub fn sanitize_interface_implementations(_tracer: Tracer) {
    // TODO: let input = tracer.trace_func(args: T, span: &'static str, func: fn(args: T) -> Result<T, Error>);
    let input: Vec<InterfaceImplementations> = vec![];
    let mut output: Vec<InterfaceImplementations> = vec![];
    for definition in input {
        let interface_uri = Uri::new(&definition.interface.get_uri());
        let implementations = definition.implementations;
        output.push(InterfaceImplementations {
            interface: interface_uri,
            implementations,
        });
    }
}
