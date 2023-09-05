pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
    for (index, num) in numbers.iter().enumerate() {
        for (index2, num2) in numbers.iter().enumerate().skip(index + 1) {
            if num + num2 == target {
                return vec![(index + 1) as i32, (index2 + 1) as i32];
            }
        }
    }
    unreachable!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_with_simple_numbers() {
        let numbers = vec![2, 7, 11, 15];
        assert_eq!(two_sum(numbers, 9), vec![1, 2]);
    }
}
