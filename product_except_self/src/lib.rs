pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
    let mut result = vec![1; nums.len()];
    let mut left = 1;
    let mut right = 1;

    nums.iter()
        .enumerate()
        .zip(nums.iter().enumerate().rev())
        .for_each(|((left_index, left_mul), (right_index, right_mul))| {
            result[left_index] *= left;
            result[right_index] *= right;
            left *= left_mul;
            right *= right_mul;
        });

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
