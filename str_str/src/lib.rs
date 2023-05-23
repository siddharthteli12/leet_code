pub fn str_str(haystack: String, needle: String) -> i32 {
    match haystack.find(&needle) {
        None => -1 as i32,
        Some(n) => n as i32,
    }
}
