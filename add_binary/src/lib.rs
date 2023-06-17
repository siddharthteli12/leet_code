pub fn add_binary(a: String, b: String) -> String {
        

        a
}


#[cfg(tests)]
mod tests {
    use super::*;
    #[test]
    fn test_with_empty_inputs() {
        assert_eq!(add_binary(String::new(), String::new()), String::new());
    }
}