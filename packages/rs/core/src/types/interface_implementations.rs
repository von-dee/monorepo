use super::Uri;
use polywrap_tracing_rs::Tracer;

#[derive(Debug)]
pub struct InterfaceImplementations<'a> {
    interface: Uri<'a>,
    implementations: Vec<Uri<'a>>,
}

pub fn sanitize_interface_implementations(_tracer: Tracer) {
    todo!()
}
