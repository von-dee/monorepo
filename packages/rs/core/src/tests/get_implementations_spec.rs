use crate::algorithms::get_implementations::get_implementations;
use crate::types::{InterfaceImplementations, Uri, UriRedirect};

#[test]
fn test_that_it_works_with_complex_redirects() {
    let interface1_uri = "w3://ens/some-interface1.eth";
    let interface2_uri = "w3://ens/some-interface2.eth";
    let interface3_uri = "w3://ens/some-interface3.eth";

    let implementation1_uri = "w3://ens/some-implementation.eth";
    let implementation2_uri = "w3://ens/some-implementation2.eth";
    let implementation3_uri = "w3://ens/some-implementation3.eth";

    let redirects: Vec<UriRedirect> = vec![
        UriRedirect {
            from: Some(Uri::new(interface1_uri)),
            to: Some(Uri::new(interface2_uri)),
        },
        UriRedirect {
            from: Some(Uri::new(implementation1_uri)),
            to: Some(Uri::new(implementation2_uri)),
        },
        UriRedirect {
            from: Some(Uri::new(implementation2_uri)),
            to: Some(Uri::new(implementation3_uri)),
        },
    ];

    let interfaces: Vec<InterfaceImplementations> = vec![
        InterfaceImplementations {
            interface: Uri::new(interface1_uri),
            implementations: vec![Uri::new(implementation1_uri), Uri::new(implementation2_uri)],
        },
        InterfaceImplementations {
            interface: Uri::new(interface2_uri),
            implementations: vec![Uri::new(implementation3_uri)],
        },
        InterfaceImplementations {
            interface: Uri::new(interface3_uri),
            implementations: vec![Uri::new(implementation3_uri)],
        },
    ];

    let get_implementations_result1 =
        get_implementations(Uri::new(interface1_uri), &redirects, &interfaces);
    let get_implementations_result2 =
        get_implementations(Uri::new(interface2_uri), &redirects, &interfaces);
    let get_implementations_result3 =
        get_implementations(Uri::new(interface3_uri), &redirects, &interfaces);

    assert_eq!(
        get_implementations_result1,
        vec![
            Uri::new(implementation1_uri),
            Uri::new(implementation2_uri),
            Uri::new(implementation3_uri)
        ]
    );
    assert_eq!(
        get_implementations_result2,
        vec![
            Uri::new(implementation1_uri),
            Uri::new(implementation2_uri),
            Uri::new(implementation3_uri)
        ]
    );
    assert_eq!(
        get_implementations_result3,
        vec![Uri::new(implementation3_uri)]
    );
}

#[test]
fn test_that_interface_implementations_are_not_redirected() {
    let interface1_uri = "w3://ens/some-interface1.eth";

    let implementation1_uri = "w3://ens/some-implementation.eth";
    let implementation2_uri = "w3://ens/some-implementation2.eth";

    let redirects: Vec<UriRedirect> = vec![UriRedirect {
        from: Some(Uri::new(implementation1_uri)),
        to: Some(Uri::new(implementation2_uri)),
    }];

    let interfaces: Vec<InterfaceImplementations> = vec![InterfaceImplementations {
        interface: Uri::new(interface1_uri),
        implementations: vec![Uri::new(implementation1_uri)],
    }];

    let get_implementations_result =
        get_implementations(Uri::new(interface1_uri), &redirects, &interfaces);

    assert_eq!(
        get_implementations_result,
        vec![Uri::new(implementation1_uri)]
    );
}
