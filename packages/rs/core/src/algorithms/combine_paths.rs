pub fn combine_paths(a: &str, b: &str) -> String {
    // Normalize all path separators
    let mut a = a.replace("\\", "/");
    let mut b = b.replace("\\", "/");

    // Append a separator if one doesn't exist
    if a.chars().last().unwrap() != '/' {
        a.push_str("/");
    }

    // Remove any leading separators
    while b.chars().nth(0).unwrap() == '/' || b.chars().nth(0).unwrap() == '.' {
        let temp_b = b.clone();
        let mut chars = temp_b.chars();
        chars.next();
        b.push_str(chars.as_str());
    }

    [a, b].concat()
}
