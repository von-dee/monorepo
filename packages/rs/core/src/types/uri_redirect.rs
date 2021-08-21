use super::Uri;
use polywrap_tracing_rs::Tracer;

#[derive(Clone)]
pub struct UriRedirect {
    from: Uri,
    to: Uri,
}

pub fn sanitize_uri_redirects(_tracer: Tracer) {
    // TODO: let input = tracer.trace_func(args: T, span: &'static str, func: fn(args: T) -> Result<T, Error>);
    let input: Vec<UriRedirect> = vec![];
    let mut output: Vec<UriRedirect> = vec![];
    for definition in input {
        let from = Uri::new(&definition.from.get_uri());
        let to = definition.to;
        output.push(UriRedirect { from, to });
    }
}
