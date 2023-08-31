use std::collections::{HashMap, HashSet};
pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
    let mut num_set: HashSet<i32> = HashSet::new();
    let mut start_map: HashMap<i32, Vec<i32>> = HashMap::new();
    for num in nums {
        num_set.insert(num);
    }

    for num in num_set.iter() {
        if num_set.contains(&(num + 1)) && !num_set.contains(&(num - 1)) {
            start_map.insert(*num, vec![*num]);
        }
    }

    for (key, value) in start_map.iter_mut() {
        let mut counter = 1;

        loop {
            if num_set.contains(&(key + counter)) {
                value.push(key + counter);
                counter += 1;
            } else {
                break;
            }
        }
    }

    start_map
        .values()
        .map(|list| list.len() as i32)
        .max()
        .unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_list() {
        let nums = vec![100,4,200,1,3,2];
        assert_eq!(longest_consecutive(nums), 4);
    }

    #[test]
    fn simple_list_1() {
        let nums = vec![100,4,105, 103,104, 1,3,2, 101, 102, 106];
        assert_eq!(longest_consecutive(nums), 7);
    }
}
