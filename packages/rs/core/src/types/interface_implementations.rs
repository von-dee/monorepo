use super::Uri;

#[derive(Clone, Debug)]
pub struct InterfaceImplementations {
    pub interface: Uri,
    pub implementations: Vec<Uri>,
}

pub fn sanitize_interface_implementations(
    input: &[InterfaceImplementations],
) -> Result<Vec<InterfaceImplementations>, &str> {
    let mut output: Vec<InterfaceImplementations> = vec![];
    for definition in input {
        let interface_uri = Uri::new(&definition.interface.get_uri().unwrap());
        let implementations = &definition.implementations;
        output.push(InterfaceImplementations {
            interface: interface_uri,
            implementations: implementations.to_vec(),
        });
    }
    Ok(output)
}
