pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
    let mut counter = 1;
    let mut pointer = match nums.get(0) {
        Some(val) => val.clone(),
        None => return 0,
    };
    for num in nums.clone() {
        if num == pointer {
            continue;
        } else {
            nums[counter as usize] = num;
            pointer = num;
            counter = counter + 1;
        }
    }
    counter
}
