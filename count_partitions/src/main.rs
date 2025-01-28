fn main() {
    println!("{:?}", count_partitions(vec![10,10,3,7,6]));
}


pub fn count_partitions(nums: Vec<i32>) -> i32 {
    let mut left = nums[0];
    let mut right: i32 = nums.iter().skip(1).sum();
    let mut result= 0;

    for num in nums.iter().skip(1) {
        if (right - left) % 2 == 0 {
            result += 1;
        }
        right -= num;
        left += num;

    }



    result

}
