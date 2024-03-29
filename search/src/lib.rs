use std::cmp::Ordering;

pub fn search(nums: Vec<i32>, target: i32) -> i32 {
    if nums.len() == 1 {
        if nums[0] == target {
            return 0;
        } else {
            return -1;
        }
    }
    let mut left = 0;
    let mut right: i32 = (nums.len() - 1) as i32;
    let mut middle = right / 2;

    while left <= right {
        match nums[middle as usize].cmp(&target) {
            Ordering::Equal => return middle,
            Ordering::Greater => {
                right = middle - 1;
                middle = (right + left) / 2;
            }
            Ordering::Less => {
                left = middle + 1;
                middle = (right + left) / 2;
            }
        }
    }
    -1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_with_simple_list() {
        let list = vec![12, 34, 56, 78, 90, 99, 120, 909, 1021];
        assert_eq!(7, search(list, 909));
    }

    #[test]
    fn test_with_simple_list1() {
        let list = vec![12, 34, 56, 78, 90, 99, 120, 909, 1021, 2023];
        assert_eq!(8, search(list, 1021));
    }

    #[test]
    fn test_with_single_item() {
        assert_eq!(-1, search(vec![-5], 11));
    }
}
