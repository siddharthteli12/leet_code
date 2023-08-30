use std::collections::HashMap;

pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
    let mut hashmap = HashMap::new();

    for str in strs {
        hashmap
            .entry(represent_in_char(&str))
            .and_modify(|list: &mut Vec<String>| list.push(str.clone()))
            .or_insert(vec![str]);
    }

    hashmap.into_values().collect()
}

fn represent_in_char(str: &str) -> [i32; 26] {
    let mut result = [0; 26];

    for byte in str.bytes() {
        result[(byte - b'a') as usize] += 1;
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_with_simple_anagram() {
        let strs = vec![
            "eat".to_string(),
            "tea".to_string(),
            "tan".to_string(),
            "ate".to_string(),
            "nat".to_string(),
            "bat".to_string(),
        ];
        assert_eq!(
            group_anagrams(strs),
            vec![
                vec!["bat".to_string()],
                vec!["nat".to_string(), "tan".to_string()],
                vec!["ate".to_string(), "eat".to_string(), "tea".to_string()]
            ]
        );
    }
}
