use std::collections::HashMap;
use std::string::ToString;

// TODO:

pub fn filter_results<T>(
    result: Option<T>,
    filter: HashMap<String, T>,
) -> Result<HashMap<String, T>, String>
where
    T: ToString + serde::Serialize + Clone,
{
    if result.is_none() {
        let stringified = serde_json::to_string(&filter).expect("Failed to stringify data");
        let custom_error = format!(
            "The result given is not an object. 
                Filters can only be given on results that are objects.\nFilter: {}",
            stringified
        );
        return Err(custom_error);
    }
    let mut filtered: HashMap<String, T> = HashMap::new();
    for key in filter.keys() {
        let res = result.clone().unwrap();
        let _ = filtered.insert(key.to_owned(), res);
    }
    Ok(filtered)
}
