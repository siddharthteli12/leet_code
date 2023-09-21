pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut result: Vec<Vec<i32>> = vec![];
    nums.sort();
    if nums[0] > 0 || *nums.last().unwrap() < 0 {
        return vec![];
    }
    for (index, num) in nums.iter().enumerate() {
        if *num > 0 {
            break;
        } else {
            let mut counter = nums.len() - 1;
            while counter > index {
                if nums[counter] < 0 {
                    break;
                } else {
                    let target = num.abs() - nums[counter];
                    if target + nums[counter] + num == 0 {
                        if let Ok(target_index) = nums.binary_search(&target) {
                            if target_index != counter && target_index != index {
                                let mut three_sum = vec![*num, target, nums[counter]];
                                three_sum.sort();
                                if !result.contains(&three_sum) {
                                    result.push(three_sum.to_vec());
                                }
                            }
                        }
                    }
                    counter -= 1;
                }
            }
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_with_simple_list() {
        let nums = vec![-1, 0, 1, 2, -1, -4];
        assert_eq!(three_sum(nums), [[-1, -1, 2], [-1, 0, 1]]);
    }
}
