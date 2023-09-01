pub fn is_valid(s: String) -> bool {
    if s.len() < 2 {
        return false;
    }
    let mut stack: Vec<char> = vec![];

    for char in s.chars() {
        match char {
            opening_value @ ('(' | '[' | '{') => {
                stack.push(opening_value);
            }
            closing_value @ (')' | ']' | '}') => {
                if let Some(char_pop) = stack.pop() {
                    match char_pop {
                        local_value @ '(' => {
                            if (local_value as u8 + 1) as char == closing_value {
                                continue;
                            } else {
                                return false;
                            }
                        }
                        local_value @ ('[' | '{') => {
                            if (local_value as u8 + 2) as char == closing_value {
                                continue;
                            } else {
                                return false;
                            }
                        }
                        _ => return false,
                    }
                } else {
                    return false;
                }
            }
            _ => {
                unreachable!()
            }
        }
    }

    stack.is_empty()
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
