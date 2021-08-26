use super::Uri;

#[derive(Clone)]
pub struct UriRedirect {
    from: Uri,
    to: Uri,
}

pub fn sanitize_uri_redirects(input: &[UriRedirect]) -> Result<Vec<UriRedirect>, &str> {
    let mut output: Vec<UriRedirect> = vec![];
    for definition in input {
        let from = Uri::new(&definition.from.get_uri());
        let to = definition.to.clone();
        output.push(UriRedirect { from, to });
    }
    Ok(output)
}
