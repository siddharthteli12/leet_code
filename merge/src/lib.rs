pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
    let mut i = nums1.len() - 1;
    for num in nums2 {
        nums1[i] = *num;
        i -= 1;
    }

    nums1.sort();
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_work_1() {
        let mut nums1 = vec![1, 2, 3, 0, 0, 0];
        let m = 3;
        let mut nums2 = vec![2, 5, 6];
        let n = 3;

        merge(&mut nums1, m, &mut nums2, n);
        assert_eq!(nums1, vec![1, 2, 2, 3, 5, 6]);
    }
}
