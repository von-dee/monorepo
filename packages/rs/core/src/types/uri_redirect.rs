use super::Uri;

#[derive(Clone, Default, Debug, PartialEq)]
pub struct UriRedirect {
    pub from: Option<Uri>,
    pub to: Option<Uri>,
}

#[derive(Clone, Default, Debug, PartialEq)]
pub struct UriRedirectArgs {
    pub from: String,
    pub to: String,
}

pub fn sanitize_uri_redirects(input: &[UriRedirectArgs]) -> Result<Vec<UriRedirect>, &str> {
    let mut output: Vec<UriRedirect> = vec![];
    for definition in input {
        let from = Uri::new(&definition.from);
        let to = Uri::new(&definition.to);
        output.push(UriRedirect {
            from: Some(from),
            to: Some(to),
        });
    }
    Ok(output)
}
