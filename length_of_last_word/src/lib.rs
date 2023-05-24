pub fn length_of_last_word(s: String) -> i32 {
    let s = s.trim();
    match s.rsplit_once(" ") {
        Some((_, last)) => last.len() as i32,
        None => s.len() as i32,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_with_empty_string() {
        let sentence = String::new();
        assert_eq!(length_of_last_word(sentence), 0);
    }

    #[test]
    fn test_with_single_string() {
        let sentence = String::from("Hello");
        assert_eq!(length_of_last_word(sentence), 5);
    }

    #[test]
    fn test_with_single_string_with_whitespace() {
        let sentence = String::from("    Hello   ");
        assert_eq!(length_of_last_word(sentence), 5);
    }

    #[test]
    fn test_with_strings() {
        let sentence = String::from("Hello how are you?");
        assert_eq!(length_of_last_word(sentence), 4);
    }

    #[test]
    fn test_with_strings_2() {
        let sentence = String::from("What    Is Going on Everywhere  ");
        assert_eq!(length_of_last_word(sentence), 10);
    }
}
