use super::Uri;

#[derive(Clone)]
pub struct UriRedirect {
    pub from: Option<Uri>,
    pub to: Option<Uri>,
}

pub fn sanitize_uri_redirects(input: &[UriRedirect]) -> Result<Vec<UriRedirect>, &str> {
    let mut output: Vec<UriRedirect> = vec![];
    for definition in input {
        let from = Uri::new(&definition.from.as_ref().unwrap().get_uri().unwrap());
        let to = Uri::new(&definition.to.as_ref().unwrap().get_uri().unwrap());
        output.push(UriRedirect {
            from: Some(from),
            to: Some(to),
        });
    }
    Ok(output)
}
