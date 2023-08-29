pub fn group_anagrams(mut strs: Vec<String>) -> Vec<Vec<String>> {
    let mut result: Vec<Vec<String>> = vec![];

    while !strs.is_empty() {
        if strs.len() == 1 {
            result.push(vec![strs.pop().unwrap()]);
        } else {
            let str1 = strs.remove(0);
            let mut local_index = 0;
            let mut anagram_list: Vec<String> = vec![];
            while local_index < strs.len() {
                if compare_anagram(&str1, &strs[local_index].clone()) {
                    anagram_list.push(strs.remove(local_index));
                } else {
                    local_index += 1;
                }
            }
            anagram_list.push(str1);
            result.push(anagram_list);
        }
    }

    result
}

fn compare_anagram(str1: &str, str2: &str) -> bool {
    let mut char_count = [0; 26];

    for char in str1.bytes() {
        char_count[(char - b'a') as usize] += 1;
    }

    for char in str2.bytes() {
        char_count[(char - b'a') as usize] -= 1;
    }
    for count in char_count {
        if count != 0 {
            return false;
        }
    }
    true
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
