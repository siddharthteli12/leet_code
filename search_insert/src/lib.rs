pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
    let mut left_index: i32 = 0;
    let mut right_index: i32 = (nums.len() - 1) as i32;
    let mut middle_index: i32;
    while left_index <= right_index {
        middle_index = (left_index + right_index) / 2;
        if nums[middle_index as usize] == target {
            return middle_index as i32;
        } else if nums[middle_index as usize] > target {
            right_index = middle_index - 1;
        } else {
            left_index = middle_index + 1;
        }
    }
    left_index as i32
}
