pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
    let length = nums.len() - 1;
    let mut result = vec![1; length + 1];
    let mut left = 1;
    let mut right = 1;
    for index in 0..=length {
        result[index] *= left;
        result[length - index] *= right;
        left *= nums[index];
        right *= nums[length - index];
    }

    result
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_with_simple_list() {
        let list = vec![1, 2, 3, 4];
        assert_eq!(product_except_self(list), vec![24, 12, 8, 6]);
    }
}
