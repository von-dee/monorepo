use crate::types::{Uri, UriRedirect};
use std::collections::HashMap;

pub fn apply_redirects(uri: &Uri, redirects: &[UriRedirect]) -> Result<Uri, String> {
    // Keep track of past redirects (from -> to) to find the final uri
    let mut redirect_from_to_map: HashMap<String, Uri> = HashMap::new();
    for redirect in redirects {
        if redirect.from.is_none() {
            let msg = format!(
                "Redirect missing the from property.\nEncountered while resolving {}",
                uri.get_uri().unwrap()
            );
            return Err(throw_error(msg, &redirect_from_to_map));
        }
        if redirect_from_to_map
            .get(&redirect.from.as_ref().unwrap().get_uri().unwrap())
            .is_some()
        {
            continue;
        }
        let _ = redirect_from_to_map.insert(
            redirect.from.as_ref().unwrap().get_uri().unwrap(),
            redirect.to.clone().unwrap(),
        );
    }
    let mut final_uri = uri.clone();
    let mut visited_uris: HashMap<String, bool> = HashMap::new();
    while redirect_from_to_map
        .get(&final_uri.get_uri().unwrap())
        .is_some()
    {
        let _ = visited_uris.insert(final_uri.get_uri().unwrap(), true);
        final_uri = redirect_from_to_map
            .get(&final_uri.get_uri().unwrap())
            .unwrap()
            .clone();
        if *visited_uris.get(&final_uri.get_uri().unwrap()).unwrap() {
            let msg = format!("Infinite loop while resolving URI {}.", uri.to_string());
            return Err(throw_error(msg, &redirect_from_to_map));
        }
    }
    Ok(final_uri)
}

fn throw_error(message: String, data: &HashMap<String, Uri>) -> String {
    let s = serde_json::to_string(data).expect("Failed to stringify data");
    format!("{}\nResolution Stack: {}", message, s)
}
