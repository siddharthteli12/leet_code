use std::{collections::HashMap, vec};

pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
    let mut result = vec![];
    let mut num_count = HashMap::<i32, i32>::new();
    for num in nums.iter() {
        if let Some(value) = num_count.get_mut(num) {
            *value += 1;
        } else {
            num_count.insert(*num, 1);
        }
    }

    let mut num_count_list: Vec<(i32, i32)> = num_count
        .iter()
        .map(|(key, value)| (*key, *value))
        .collect();
    let mut sorted_list = sort_list(&mut num_count_list);
    for _ in 0..k {
        result.push(sorted_list.pop().unwrap().0);
    }
    result
}

fn sort_list(target: &mut Vec<(i32, i32)>) -> Vec<(i32, i32)> {
    if target.len() == 1 {
        return vec![target.pop().unwrap()];
    }
    let mid = target.len() / 2;
    let mut left = sort_list(&mut target.drain(..mid).collect());
    let mut right = sort_list(&mut target.drain(..).collect());
    let mut result = vec![];
    while !left.is_empty() && !right.is_empty() {
        if left[0].1 >= right[0].1 {
            result.push(right.remove(0));
        } else {
            result.push(left.remove(0));
        }
    }

    result.append(if !left.is_empty() {
        &mut left
    } else {
        &mut right
    });
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_with_simple_list1() {
        let num_list = vec![8, 100, 123, 8, 7, 70, 8, 100, 8, 90, 8, 100];
        let k = 2;
        assert_eq!(top_k_frequent(num_list, k), vec![8, 100]);
    }

    #[test]
    fn test_with_simple_list2() {
        let num_list = vec![1, 2, 2, 3, 3, 3, 4, 4, 4, 4, 5, 5, 5, 5, 5, 5];
        let k = 5;
        assert_eq!(top_k_frequent(num_list, k), vec![5, 4, 3, 2, 1]);
    }
}
