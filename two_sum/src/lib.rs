use std::collections::HashMap;
pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut map = HashMap::new();
    for (i, num) in nums.iter().enumerate() {
        map.insert(num, i as i32);
    }
    for i in 0..nums.len() {
        if let Some(val) = map.get(&(target - nums[i])) {
            if *val != i as i32 {
                return vec![i as i32, *val];
            }
        }
    }

    return vec![0, 0];
}
