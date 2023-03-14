use std::collections::HashMap;
pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut map = HashMap::new();
    for (i, num) in nums.iter().enumerate() {
        map.insert(num, i as i32);
    }
    for i in 0..nums.len() {
        if let Some(val) = map.get(&(target - nums[i])) {
            if *val != i as i32 {
                return vec![i as i32, *val];
            }
        }
    }

    return vec![0, 0];
}

#[cfg(test)]
mod tests {
    use super::*;

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
        assert_eq!(result, vec![1, 6]);
    }

    #[test]
    fn test_two_sum_with_negative_element() {
        let result = two_sum(vec![10, 1, 8, 1, -2, 12, -2, 5, 9], 3);
        assert_eq!(result, vec![4, 7]);
    }
}
