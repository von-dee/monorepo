use crate::algorithms::apply_redirects::apply_redirects;
use crate::types::uri::Uri;
use crate::types::uri_redirect::UriRedirect;

#[test]
fn it_works_with_the_typical_use_case() {
    let uri1 = "w3://ens/some-uri1.eth";
    let uri2 = "w3://ens/some-uri2.eth";

    let input_args = UriRedirect {
        from: Some(Uri::new(uri1)),
        to: Some(Uri::new(uri2)),
    };

    let redirects: Vec<UriRedirect> = vec![input_args];
    let redirected_uri = apply_redirects(&Uri::new(uri1), &redirects).unwrap();

    assert!(Uri::equals(&redirected_uri, &Uri::new(uri2)));
}

#[test]
fn it_works_with_the_redirect_stack_overrides() {
    let uri1 = "w3://ens/some-uri1.eth";
    let uri2 = "w3://ens/some-uri2.eth";
    let uri3 = "w3://ens/some-uri3.eth";

    let input_args1 = UriRedirect {
        from: Some(Uri::new(uri1)),
        to: Some(Uri::new(uri2)),
    };
    let input_args2 = UriRedirect {
        from: Some(Uri::new(uri1)),
        to: Some(Uri::new(uri3)),
    };

    let redirects: Vec<UriRedirect> = vec![input_args1, input_args2];
    let redirected_uri = apply_redirects(&Uri::new(uri1), &redirects).unwrap();

    assert!(Uri::equals(&redirected_uri, &Uri::new(uri2)));
}

#[test]
#[should_panic]
fn it_cannot_redirect_to_self() {
    let uri = "w3://ens/some-uri.eth";

    let input_args = UriRedirect {
        from: Some(Uri::new(uri)),
        to: Some(Uri::new(uri)),
    };

    let redirects: Vec<UriRedirect> = vec![input_args];
    apply_redirects(&Uri::new(uri), &redirects).expect("/Infinite loop while resolving URI/");
}
