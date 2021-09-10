use regex::Regex;
use std::string::ToString;

pub fn file<T: ToString>(path: T) -> bool {
    let re = Regex::new(r"/^((./|../)[^/ ]*)+/?$/").unwrap();
    let path = path.to_string();
    let valid_path_match = re.find(&path);

    let mut result = false;
    if valid_path_match.is_some() {
        result = valid_path_match.unwrap().as_str().len() == path.len();
    }
    result
}

// This may not be necessary
pub fn docker_image_name<T: ToString>(name: T) -> bool {
    let name = name.to_string();
    !name.is_empty()
}

pub fn docker_file_name<T: ToString>(value: T) -> bool {
    let value = value.to_string();
    !value.is_empty() && file(value.clone()) && value.contains("Dockerfile")
}

pub fn docker_image_id<T: ToString>(value: T) -> bool {
    let value = value.to_string();
    !value.is_empty() && value.contains("sha256:")
}

pub fn wasm_language<T: ToString>(language: T) -> bool {
    let language = language.to_string();
    !language.is_empty() && (language.as_str() == "interface" || language.contains("wasm/"))
}

pub fn plugin_language<T: ToString>(language: T) -> bool {
    let language = language.to_string();
    !language.is_empty() && language.contains("plugin/")
}

pub fn image_file<T: ToString>(file_path: T) -> bool {
    let file_path = file_path.to_string();
    let re = Regex::new(r"/(.svg|.png)$/").unwrap();
    let f = file_path.clone();
    let result = re.find(&f).unwrap();

    !file_path.is_empty() && file(file_path) && !result.as_str().is_empty()
}

pub fn website_url<T: ToString>(url: T) -> bool {
    let url = url.to_string();
    !url.is_empty() && valid_url(&url)
}

pub fn graphql_file<T: ToString>(file_path: T) -> bool {
    let file_path = file_path.to_string();
    let re = Regex::new(r"/(.graphql)$/").unwrap();
    let f = file_path.clone();
    let result = re.find(&f).unwrap();

    !file_path.is_empty() && file(file_path) && !result.as_str().is_empty()
}

pub fn json_file<T: ToString>(file_path: T) -> bool {
    let file_path = file_path.to_string();
    let re = Regex::new(r"/(.json)$/").unwrap();
    let f = file_path.clone();
    let result = re.find(&f).unwrap();

    !file_path.is_empty() && file(file_path) && !result.as_str().is_empty()
}

pub fn yaml_file<T: ToString>(file_path: T) -> bool {
    let file_path = file_path.to_string();
    let re = Regex::new(r"/(.yaml)$/").unwrap();
    let f = file_path.clone();
    let result = re.find(&f).unwrap();

    !file_path.is_empty() && file(file_path) && !result.as_str().is_empty()
}

fn valid_url(s: &str) -> bool {
    let pattern = Regex::new(
        r"^(https?:\\/\\/)?
        ((([a-z\\d]([a-z\\d-]*[a-z\\d])*)\\.)+[a-z]{2,}|
        ((\\d{1,3}\\.){3}\\d{1,3}))
        (\\:\\d+)?(\\/[-a-z\\d%_.~+]*)*
        (\\?[;&a-z\\d%_.~+=-]*)?
        (\\#[-a-z\\d_]*)?$
        i",
    )
    .unwrap();
    pattern.is_match(s)
}
