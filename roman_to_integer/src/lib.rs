pub fn roman_to_int(s: String) -> i32 {
    let mut result;
    let mut iter = s.chars();

    if let Some(mut previous_roman) = iter.next() {
        result = convert_roman_to_num(previous_roman);
        for roman_char in iter {
            match (previous_roman, roman_char) {
                ('I', 'V' | 'X') => {
                    calculate_result_utility(previous_roman, roman_char, &mut result);
                }
                ('X', 'L' | 'C') => {
                    calculate_result_utility(previous_roman, roman_char, &mut result);
                }
                ('C', 'D' | 'M') => {
                    calculate_result_utility(previous_roman, roman_char, &mut result);
                }
                (_, _) => result += convert_roman_to_num(roman_char),
            }
            previous_roman = roman_char;
        }
        result
    } else {
        0
    }
}

pub fn convert_roman_to_num(roman_char: char) -> i32 {
    match roman_char {
        'I' => 1,
        'V' => 5,
        'X' => 10,
        'L' => 50,
        'C' => 100,
        'D' => 500,
        'M' => 1000,
        _ => unreachable!(),
    }
}

fn calculate_result_utility(previous_roman: char, roman_char: char, result: &mut i32) {
    let value = convert_roman_to_num(roman_char) - 2 * convert_roman_to_num(previous_roman);
    *result += value;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_roman() {
        let result = roman_to_int("XII".to_string());
        assert_eq!(result, 12);
    }

    #[test]
    fn test_simple_roman_1() {
        let result = roman_to_int("III".to_string());
        assert_eq!(result, 3);
    }

    #[test]
    fn test_complex_roman_1() {
        let result = roman_to_int("LVIII".to_string());
        assert_eq!(result, 58);
    }

    #[test]
    fn test_complex_roman_2() {
        let result = roman_to_int("MCMXCIV".to_string());
        assert_eq!(result, 1994);
    }
}
