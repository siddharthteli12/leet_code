use std::collections::HashSet;
pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
    if nums.len() < 2 {
        return nums.len() as i32;
    }
    let num_set: HashSet<i32> = nums.into_iter().collect();
    let mut max_value = 1;

    for num in num_set.iter() {
        if !num_set.contains(&(num - 1)) {
            let mut counter = 1;
            while num_set.contains(&(num + counter)) {
                counter += 1;
                if max_value < counter {
                    max_value = counter;
                }
            }
        }
    }
    max_value
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_list() {
        let nums = vec![100, 4, 200, 1, 3, 2];
        assert_eq!(longest_consecutive(nums), 4);
    }

    #[test]
    fn simple_list_1() {
        let nums = vec![100, 4, 105, 103, 104, 1, 3, 2, 101, 102, 106];
        assert_eq!(longest_consecutive(nums), 7);
    }

    #[test]
    fn single_item() {
        let nums = vec![0];
        assert_eq!(longest_consecutive(nums), 1);
    }

    #[test]
    fn repeated_item() {
        let nums = vec![0, 0];
        assert_eq!(longest_consecutive(nums), 1);
    }
}
