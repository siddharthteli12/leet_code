pub fn pivot_index(nums: Vec<i32>) -> i32 {
    let mut index = -1;
    let mut total_sum = nums.iter().sum();
    let mut left_sum = 0;

    for i in 0..nums.len() {
        left_sum += nums[i];
        if left_sum == total_sum {
            index = i as i32;
            break;
        }
        total_sum -= nums[i];
    }
    index
}

// -1,-1,0,0,-1,-1
//

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn should_work() {
        let nums = vec![1, 7, 3, 6, 5, 6];
        assert_eq!(pivot_index(nums), 3);
    }
}
