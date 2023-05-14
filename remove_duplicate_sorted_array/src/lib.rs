pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
    let mut counter = 1;
    let mut non_duplicate_list: Vec<i32> = vec![];
    let mut pointer = match nums.get(0) {
        Some(val) => {
            non_duplicate_list.push(val.clone());
            val.clone()
        }
        None => return 0,
    };
    for num in nums.clone() {
        if num == pointer {
            continue;
        } else {
            non_duplicate_list.push(num);
            pointer = num;
            counter = counter + 1;
        }
    }
    nums.clear();
    nums.append(&mut non_duplicate_list);
    counter
}
