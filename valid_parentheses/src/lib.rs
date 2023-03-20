pub fn is_valid(s: String) -> bool {
    if s.len() <= 1 || s.len() % 2 != 0 {
        println!("Inside first if, s.len --, {:?}", s.len());
        return false;
    }
    let s_vec: Vec<char> = s.chars().collect();
    let mut stack: Vec<char> = Vec::new();
    for item in s_vec.iter() {
        match item {
            '(' | '{' | '[' => stack.push(*item),
            ')' | '}' | ']' => match stack.pop() {
                Some(n) => {
                    if match_closing_parenthese(n, *item) {
                        continue;
                    } else {
                        return false;
                    }
                }
                None => return false,
            },
            _ => {
                unreachable!();
            }
        }
    }

    if stack.is_empty() {
        return true;
    } else {
        return false;
    }
}

pub fn match_closing_parenthese(parenthese1: char, parenthese2: char) -> bool {
    print!("Par1 {:}, Par2 {:} .", parenthese1, parenthese2);
    match (parenthese1, parenthese2) {
        ('(', ')') => true,
        ('{', '}') => true,
        ('[', ']') => true,
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_parentheses() {
        let result = is_valid(String::from("()"));
        assert_eq!(result, true);
    }

    #[test]
    fn test_all_parentheses() {
        let result = is_valid(String::from("()[]{}"));
        assert_eq!(result, true);
    }

    #[test]
    fn test_invalid_parentheses() {
        let result = is_valid(String::from("(][]{}"));
        assert_eq!(result, false);
    }

    #[test]
    fn test_empty_parentheses() {
        let result = is_valid(String::from(""));
        assert_eq!(result, false);
    }

    #[test]
    fn test_single_parentheses() {
        let result = is_valid(String::from("{"));
        assert_eq!(result, false);
    }

    #[test]
    fn test_odd_parentheses() {
        let result = is_valid(String::from("{}{"));
        assert_eq!(result, false);
    }
}
