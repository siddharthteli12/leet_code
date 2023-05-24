pub fn length_of_last_word(s: String) -> i32 {
    let string_vec = s.split_whitespace().collect::<Vec<&str>>();
    match string_vec.last() {
        Some(str) => str.len() as i32,
        None => 0 as i32,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
