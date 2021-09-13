use super::Uri;

#[derive(Clone, Debug, PartialEq)]
pub struct InterfaceImplementations {
    pub interface: Uri,
    pub implementations: Vec<Uri>,
}

#[derive(Clone, Debug)]
pub struct InterfaceImplementationsArgs {
    pub interface: String,
    pub implementations: Vec<String>,
}

pub fn sanitize_interface_implementations(
    input: &[InterfaceImplementationsArgs],
) -> Result<Vec<InterfaceImplementations>, &str> {
    let mut output: Vec<InterfaceImplementations> = vec![];
    for definition in input {
        let interface_uri = Uri::new(&definition.interface);
        let implementations: Vec<Uri> = definition
            .clone()
            .implementations
            .iter_mut()
            .map(|entry| Uri::new(entry))
            .collect();
        output.push(InterfaceImplementations {
            interface: interface_uri,
            implementations,
        });
    }
    Ok(output)
}
