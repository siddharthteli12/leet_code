pub fn rob(nums: Vec<i32>) -> i32 {
    let mut nums = nums;


    for i in 0..nums.len() {
        if i < 2 {
            continue;
        } else {
            let max = nums[0..(i  - 1)].iter().max().cloned().unwrap_or_default();
            nums[i] += max;
        }
    }

    

   
   nums.iter().max().cloned().unwrap_or_default()    
}


#[cfg(test)]
mod test {
    use crate::rob;

    #[test]
    fn rob_test() {
        assert_eq!(4, rob(vec![1,2,3,1]));
        assert_eq!(12, rob(vec![2,7,9,3,1]));
    }
}