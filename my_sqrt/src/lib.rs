pub fn my_sqrt(x: i32) -> i32 {
    // Using binary search to find sqrt of x.

    // Lower value is 1.
    // Upper value is sq root of max. value of i32.
    let (mut lower, mut upper) = (1, 46340);
    let mut mid: i32;

    while lower <= upper {
        mid = (lower + upper) / 2;
        if mid * mid == x {
            return mid;
        } else if mid * mid < x {
            lower = mid + 1;
        } else if mid * mid > x {
            upper = mid - 1;
        }
    }
    return upper;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_with_zero_value() {
        assert_eq!(my_sqrt(0), 0);
    }

    #[test]
    fn test_with_one_value() {
        assert_eq!(my_sqrt(1), 1);
    }

    #[test]
    fn test_with_random_value_1() {
        assert_eq!(my_sqrt(3), 1);
    }

    #[test]
    fn test_with_random_value_3() {
        assert_eq!(my_sqrt(8), 2);
    }

    #[test]
    fn test_with_random_value_4() {
        assert_eq!(my_sqrt(923), 30);
    }

    #[test]
    fn test_with_random_value_5() {
        assert_eq!(my_sqrt(11223), 105);
    }

    #[test]
    fn test_with_random_value_6() {
        assert_eq!(my_sqrt(73931), 271);
    }

    #[test]
    fn test_with_random_value_7() {
        assert_eq!(my_sqrt(2147395599), 46339);
    }
}
