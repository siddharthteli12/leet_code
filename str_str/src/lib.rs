pub fn str_str(haystack: String, needle: String) -> i32 {
    haystack
        .find(&needle)
        .map(|val| val as i32)
        .unwrap_or_else(|| -1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_with_empty_strings() {
        let haystack = String::new();
        let needle = String::new();
        assert_eq!(str_str(haystack, needle), 0);
    }

    #[test]
    fn test_without_substring() {
        let haystack = String::from("Siddharth");
        let needle = String::from("Teli");
        assert_eq!(str_str(haystack, needle), -1);
    }

    #[test]
    fn test_with_simple_strings() {
        let haystack = String::from("Siddharth");
        let needle = String::from("Sid");
        assert_eq!(str_str(haystack, needle), 0);
    }

    #[test]
    fn test_with_simple_strings_2() {
        let haystack = String::from("Siddharth");
        let needle = String::from("dha");
        assert_eq!(str_str(haystack, needle), 3);
    }
}
