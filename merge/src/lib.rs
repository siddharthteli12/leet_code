pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
    let nums1_copy = nums1[..m as usize].to_vec();
    nums1.clear();
    let mut i = 0;
    let mut j = 0;

    while i < m as usize && j < n as usize {
        if nums1_copy[i] < nums2[j] {
            nums1.push(nums1_copy[i]);
            i += 1;
        } else {
            nums1.push(nums2[j]);
            j += 1;
        }
    }

    if i < m as usize {
        nums1.extend_from_slice(&nums1_copy[i..m as usize]);
    } else {
        nums1.extend_from_slice(&nums2[j..n as usize]);
    }
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
