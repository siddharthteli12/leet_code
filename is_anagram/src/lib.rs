use std::collections::HashMap;

pub fn is_anagram(s: String, t: String) -> bool {
    let mut char_map = HashMap::<char, u16>::new();
    for key in s.chars() {
        if let Some(value) = char_map.get_mut(&key) {
            *value += 1;
        } else {
            char_map.insert(key, 1);
        }
    }
    for key in t.chars() {
        if let Some(value) = char_map.get_mut(&key) {
            match value {
                1 => {
                    char_map.remove(&key);
                }
                0 => {
                    return false;
                }
                _ => {
                    *value -= 1;
                }
            }
        } else {
            return false;
        }
    }
    char_map.is_empty()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_with_anagram() {
        let s = String::from("abc");
        let t = String::from("bca");
        assert_eq!(is_anagram(s, t), true);
    }

    #[test]
    fn test_with_anagram_2() {
        let s = String::from("night");
        let t = String::from("thing");
        assert_eq!(is_anagram(s, t), true);
    }

    #[test]
    fn test_without_anagram() {
        let s = String::from("abbc");
        let t = String::from("bca");
        assert_eq!(is_anagram(s, t), false);
    }

    #[test]
    fn test_without_anagram_2() {
        let s = String::from("headphone");
        let t = String::from("phoneheads");
        assert_eq!(is_anagram(s, t), false);
    }
}
