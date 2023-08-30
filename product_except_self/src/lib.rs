pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
    let mut result = vec![];
    let mut mul = 1;
    nums.iter().for_each(|num| {
        result.push(mul);
        mul *= num;
    });
    mul = 1;
    nums.iter().enumerate().rev().for_each(|(index, num)| {
        let value = result.get_mut(index).unwrap();
        *value *= mul;
        mul *= num;
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
