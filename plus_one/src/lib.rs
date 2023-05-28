use std::ops::Add;

pub fn plus_one(mut digits: Vec<i32>) -> Vec<i32> {
    for (index, digit) in digits.clone().iter().enumerate().rev() {
        let digit = digit.add(1);
        if digit < 10 {
            digits[index] = digit;
            return digits;
        } else {
            digits[index] = 0;
            continue;
        }
    }
    digits.push(1);
    digits.reverse();
    digits
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_with_empty_list() {
        assert_eq!(plus_one(vec![]), vec![1]);
    }

    #[test]
    fn test_with_simple_list_1() {
        assert_eq!(plus_one(vec![1, 2, 3]), vec![1, 2, 4]);
    }

    #[test]
    fn test_with_simple_list_2() {
        assert_eq!(plus_one(vec![1, 2, 3, 9]), vec![1, 2, 4, 0]);
    }

    #[test]
    fn test_with_complex_list() {
        assert_eq!(plus_one(vec![9, 9, 9]), vec![1, 0, 0, 0]);
    }
}
