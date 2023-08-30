use std::collections::HashMap;

pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
    let mut hashmap = HashMap::<Vec<char>, Vec<String>>::new();

    for str in strs {
        let mut s_vec: Vec<char> = str.chars().collect();
        s_vec.sort();
        hashmap
            .entry(s_vec)
            .and_modify(|list| list.push(str.clone()))
            .or_insert(vec![str]);
    }

    hashmap.into_values().collect()
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
