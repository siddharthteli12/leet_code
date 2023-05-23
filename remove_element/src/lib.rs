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
