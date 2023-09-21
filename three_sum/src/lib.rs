use std::collections::HashSet;
pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut result = HashSet::<(i32, i32, i32)>::new();
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
                    let target = -(num + nums[counter]);
                    if let Ok(target_index) = nums.binary_search(&target) {
                        if target_index != counter && target_index != index {
                            result.insert(sort_three_sum(*num, target, nums[counter]));
                        }
                    }
                    counter -= 1;
                }
            }
        }
    }

    result
        .iter()
        .map(|(first, sec, third)| vec![*first, *sec, *third])
        .collect()
}

fn sort_three_sum(x: i32, y: i32, z: i32) -> (i32, i32, i32) {
    match x > y {
        true => (y, x, z),
        false => match y > z {
            true => (x, z, y),
            false => (x, y, z),
        },
    }
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
