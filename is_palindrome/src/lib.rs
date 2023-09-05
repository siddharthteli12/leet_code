fn is_palindrome(s: String) -> bool {
    let s_new = s
        .chars()
        .filter(|char| char.is_alphanumeric())
        .map(|char| char.to_ascii_lowercase())
        .collect::<Vec<char>>();
    if s_new.len() < 2 {
        return true;
    }
    let mut left = 0;
    let mut right = s_new.len() - 1;
    while left < right {
        if s_new[left] == s_new[right] {
            left += 1;
            right -= 1;
            continue;
        } else {
            return false;
        }
    }
    true
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_non_palindrome() {
        let s = "Hello".to_string();
        assert_eq!(is_palindrome(s), false);
    }

    #[test]
    fn test_with_palindrome() {
        let s = "redder".to_string();
        assert_eq!(is_palindrome(s), true);
    }

    #[test]
    fn test_with_non_alphabet() {
        let s = "A man, a plan, a canal: Panama".to_string();
        assert_eq!(is_palindrome(s), true);
    }

    #[test]
    fn test_with_whitespace() {
        let s = " ".to_string();
        assert_eq!(is_palindrome(s), true);
    }
}
