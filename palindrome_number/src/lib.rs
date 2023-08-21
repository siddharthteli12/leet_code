pub fn is_palindrome(x: i32) -> bool {
    let num_string = x.to_string();
    num_string.chars().rev().collect::<String>() == num_string
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_not_palindrome() {
        let result = is_palindrome(10001000);
        assert_eq!(result, false);
    }

    #[test]
    fn test_palindrome() {
        let result = is_palindrome(112211);
        assert_eq!(result, true);
    }
}
