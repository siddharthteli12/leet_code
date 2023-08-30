pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
    let mut result = vec![];
    for (index, _) in nums.iter().enumerate() {
        result.push(
            nums.iter()
                .enumerate()
                .filter(|&(i, _)| i != index)
                .map(|a| a.1)
                .product(),
        );
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
