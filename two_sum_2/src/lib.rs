pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
    for (index, num) in numbers.iter().enumerate() {
        let compliment = target - num;
        let mut start = 0;
        let mut end = numbers.len();
        if compliment >= *num {
            start = index + 1;
        } else {
            end = index;
        }
        if let Ok(index2) = numbers[start..end].binary_search(&compliment) {
            return vec![(index + 1) as i32, (index + index2 + 2) as i32];
        }
    }
    unreachable!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_with_simple_numbers() {
        let numbers = vec![2, 7, 11, 15];
        assert_eq!(two_sum(numbers, 9), vec![1, 2]);
    }

    #[test]
    fn test_with_simple_numbers_2() {
        let numbers = vec![0, 0, 3, 4];
        assert_eq!(two_sum(numbers, 0), vec![1, 2]);
    }
}
