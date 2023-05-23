pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
    let mut result_vec: Vec<i32> = vec![];
    let mut counter = 0;
    for num in &mut *nums {
        if *num != val {
            counter = counter + 1;
            result_vec.push(num.clone());
        }
    }

    if counter > 0 {
        *nums = result_vec;
    }

    counter
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_with_empty_list() {
        let mut test_vec: Vec<i32> = vec![];
        let value = 10;
        assert_eq!(remove_element(&mut test_vec, value), 0);
        assert_eq!(test_vec, vec![]);
    }

    #[test]
    fn test_with_simple_list() {
        let mut test_vec: Vec<i32> = vec![1, 2, 3, 2, 5];
        let value = 2;
        assert_eq!(remove_element(&mut test_vec, value), 3);
        assert_eq!(test_vec, vec![1, 3, 5]);
    }

    #[test]
    fn test_with_complex_list() {
        let mut test_vec: Vec<i32> = vec![99, 12, 223, 21, 12, 12, 34, 21, 90, 90];
        let value = 12;
        assert_eq!(remove_element(&mut test_vec, value), 7);
        assert_eq!(test_vec, vec![99, 223, 21, 34, 21, 90, 90]);
    }

    #[test]
    fn test_list_without_value() {
        let mut test_vec: Vec<i32> = vec![99, 12, 223, 21, 12, 12, 34, 21, 90, 90];
        let value = 999;
        assert_eq!(remove_element(&mut test_vec, value), 10);
        assert_eq!(test_vec, vec![99, 12, 223, 21, 12, 12, 34, 21, 90, 90]);
    }
}
