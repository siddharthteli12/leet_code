pub fn is_palindrome(x: i32) -> bool {
    if x.is_negative() {
        return false;
    }
    let mut number = x;
    let mut count = 0;
    // Count total digits.
    while number != 0 {
        number = number / 10;
        count += 1;
    }
    // Reversing number.
    let mut rev_num = 0;
    let mut number = x;
    for i in (0..count).rev() {
        rev_num = rev_num + (number % 10) * 10_i32.pow(i);
        number = number / 10;
    }

    if rev_num == x {
        return true;
    } else {
        return false;
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_not_palindrome() {
        let result = is_palindrome(10001000);
        assert_eq!(result, false);
    }

    #[test]
    fn test_palindrome() {
        let result = is_palindrome(112211);
        assert_eq!(result, true);
    }
}
