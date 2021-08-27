use std::collections::HashMap;

pub fn filter_results<T: serde::Serialize + Clone>(
    result: Option<T>,
    filter: HashMap<String, Option<T>>,
) -> Result<HashMap<String, Option<T>>, String> {
    if result.is_none() {
        let stringified = serde_json::to_string(&filter).expect("Failed to stringify data");
        let custom_error = format!(
            "The result given is not an object. 
                Filters can only be given on results that are objects.\nFilter: {}",
            stringified
        );
        return Err(custom_error);
    }
    let mut filtered: HashMap<String, Option<T>> = HashMap::new();
    for key in filter.keys() {
        if result.is_some() {
            let res = result.clone();
            let _ = filtered.insert(key.to_string(), res);
        } else {
            let _ = filtered.insert(key.to_string(), None);
        }
    }
    Ok(filtered)
}
