pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
    nums.binary_search(&target)
        .map(|index| index as i32)
        .unwrap_or_else(|err_index| err_index as i32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_with_target_element() {
        let test_list = vec![
            100, 213, 215, 300, 314, 512, 829, 2301, 9000, 9100, 9807, 9999,
        ];
        let target = 829;

        assert_eq!(search_insert(test_list, target), 6);
    }

    #[test]
    fn test_without_target_element() {
        let test_list = vec![
            100, 213, 215, 300, 314, 512, 829, 2301, 9000, 9100, 9807, 9999,
        ];
        let target = 830;

        assert_eq!(search_insert(test_list, target), 7);
    }

    #[test]
    fn test_without_target_element_2() {
        let test_list = vec![1, 3, 5, 6];
        let target = 0;

        assert_eq!(search_insert(test_list, target), 0);
    }
}
