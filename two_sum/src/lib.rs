use std::collections::HashMap;

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut index_to_value: HashMap<i32, usize> = HashMap::new();

    for (index, num) in nums.iter().enumerate() {
        if let Some(complimemt) = index_to_value.get(&(target - num)) {
            return vec![*complimemt as i32, index as i32];
        }
        index_to_value.insert(*num, index);
    }

    nums
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn empty_list() {
        let nums = vec![];
        assert_eq!(two_sum(nums, 0), vec![])
    }

    #[test]
    fn test_with_positive_num() {
        let nums = vec![20, 9, 11, 18, 1, 2];
        let target = 10;
        assert_eq!(two_sum(nums, target), vec![1, 4]);
    }

    #[test]
    fn test_with_positive_num_2() {
        let nums = vec![3, 2, 4];
        let target = 6;
        assert_eq!(two_sum(nums, target), [1, 2]);
    }

    #[test]
    fn test_with_negative_num() {
        let nums = vec![-20, -9, 11, 18, -1, 2];
        let target = -10;
        assert_eq!(two_sum(nums, target), vec![1, 4]);
    }

    #[test]
    fn test_two_sum_with_sorted_ascending_list() {
        let result = two_sum(vec![1, 2, 3, 4, 5, 6, 7, 8], 3);
        assert_eq!(result, vec![0, 1]);
    }

    #[test]
    fn test_two_sum_with_sorted_decending_list() {
        let result = two_sum(vec![8, 7, 6, 5, 4, 3, 2, 1], 3);
        assert_eq!(result, vec![6, 7]);
    }

    #[test]
    fn test_two_sum_with_unsorted_list() {
        let result = two_sum(vec![10, 15, 8, 1, 2, 4, 12, 5, 9], 3);
        assert_eq!(result, vec![3, 4]);
    }

    #[test]
    fn test_two_sum_with_duplicate_element() {
        let result = two_sum(vec![10, 1, 8, 1, 2, 12, 2, 5, 9], 3);
        assert_eq!(result, vec![3, 4]);
    }

    #[test]
    fn test_two_sum_with_negative_element() {
        let result = two_sum(vec![10, 1, 8, 1, -2, 12, -2, 5, 9], 3);
        assert_eq!(result, vec![6, 7]);
    }
}
