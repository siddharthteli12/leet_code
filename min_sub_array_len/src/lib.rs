pub fn min_sub_array_len(target: i32, nums: Vec<i32>) -> i32 {
    let mut min = i32::MAX;
    let mut left = 0;
    let mut sum = 0;

    // `Input`: [1,4,4]
    for right in 0..nums.len() {
        sum += nums[right];
        if sum >= target {
            let neg = (right - left) as i32;
            min = min.min(neg + 1);

            // Start reducing the solution now.
            while left <= right {
                let temp = (sum as i32) - nums[left];
                if temp >= target {
                    sum = temp;
                    left += 1;
                    min = min.min((right - left) as i32 + 1);
                } else {
                    break;
                }
            }
        }
    }

    if min == i32::MAX {
        0
    } else {
        min
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn should_work() {
        let nums = vec![2, 3, 1, 2, 4, 3];
        let target = 7;
        assert_eq!(min_sub_array_len(target, nums), 2);
    }

    #[test]
    fn should_work2() {
        let nums = vec![1, 4, 4];
        let target = 4;
        assert_eq!(min_sub_array_len(target, nums), 1);
    }
}
