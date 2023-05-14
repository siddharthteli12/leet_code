pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
    let mut counter = 1;
    let mut pointer = match nums.get(0) {
        Some(val) => val.clone(),
        None => return 0,
    };
    for num in nums.clone() {
        if num == pointer {
            continue;
        } else {
            nums[counter as usize] = num;
            pointer = num;
            counter = counter + 1;
        }
    }
    counter
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_empty_list() {
        assert_eq!(remove_duplicates(&mut vec![]), 0);
    }

    #[test]
    fn test_without_duplicate_items() {
        let mut sample_vec = vec![1, 2, 3, 4];
        assert_eq!(remove_duplicates(&mut sample_vec), 4);
        assert_eq!(sample_vec, vec![1, 2, 3, 4]);
    }

    #[test]
    fn test_with_duplicate_items() {
        let mut sample_vec = vec![1, 1, 2, 3, 4, 4, 4, 5, 5];
        assert_eq!(remove_duplicates(&mut sample_vec), 5);
        assert_eq!(sample_vec, vec![1, 2, 3, 4, 5, 4, 4, 5, 5]);
    }

    #[test]
    fn test_with_duplicate_items_2() {
        let mut sample_vec = vec![1, 2, 3, 3, 3, 3, 70, 70, 71, 71, 101];
        assert_eq!(remove_duplicates(&mut sample_vec), 6);
        assert_eq!(sample_vec, vec![1, 2, 3, 70, 71, 101, 70, 70, 71, 71, 101]);
    }
}
