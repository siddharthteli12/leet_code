use std::ops::Add;

pub fn plus_one(digits: Vec<i32>) -> Vec<i32> {
    let mut result_vec: Vec<i32> = digits.clone();
    for (index, digit) in digits.iter().enumerate().rev() {
        let digit = digit.add(1);
        if digit < 10 {
            result_vec[index] = digit;
            return result_vec;
        } else {
            result_vec[index] = 0;
            continue;
        }
    }
    result_vec.push(1);
    result_vec.reverse();
    result_vec
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
