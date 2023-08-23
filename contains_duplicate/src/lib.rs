use std::collections::HashSet;

pub fn contains_duplicate(nums: Vec<i32>) -> bool {
    let mut nums_hash = HashSet::<i32>::new();

    for num in nums {
        if nums_hash.contains(&num) {
            return true;
        } else {
            nums_hash.insert(num);
        }
    }
    false
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_with_distinct_elements() {
        let nums = vec![1, 2, 3, 4, 5, 6, 100, 240, 221, 561];
        assert_eq!(contains_duplicate(nums), false);
    }
    #[test]
    fn test_with_duplicate_elements() {
        let nums = vec![1, 2, 3, 4, 240, 6, 100, 240, 221, 561];
        assert_eq!(contains_duplicate(nums), true);
    }
}
