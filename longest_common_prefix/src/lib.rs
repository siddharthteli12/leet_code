pub fn longest_common_prefix(strs: Vec<String>) -> String {
        let mut current_common_prefix = String::new();
        let mut previous_common_prefix = strs[0].clone();
        for i in 0..strs.len() - 1 {
            current_common_prefix = find_common_prefix(strs[i].clone(), strs[i + 1].clone());
            if current_common_prefix.len() == 0 {
                return String::from("");
            }
            else {
                previous_common_prefix =  find_common_prefix(previous_common_prefix.clone(), current_common_prefix.clone());
                if previous_common_prefix.len() == 0 {
                    return String::from("");
                }
            }

        }

        return previous_common_prefix;
}

pub fn find_common_prefix(str1: String, str2: String) -> String {   
        let mut char_vec1: Vec<char> = vec![];
        let mut char_vec2: Vec<char> = vec![];
        let mut count = 0;
        if str1.len() > str2.len() {
        char_vec1 = str2.chars().collect();
        char_vec2 = str1.chars().collect();
        }
        else {
            char_vec1 = str1.chars().collect();
            char_vec2 = str2.chars().collect();
        }

        println!(" Before for loop, Breaking value --char_vec1 {:?},char_vec2 {:?}", char_vec1, char_vec2);
        for i in 0..char_vec1.len() {

            if i == 0 {
                if char_vec1[0] == char_vec2[0] {
                    count = 1;
                    continue;
                }
                else {
                    break;
                }
            }
            println!("Breaking value --char_vec1 {:?},char_vec2 {:?}, value of i {:}", char_vec1, char_vec2, i);
            if char_vec1[i] == char_vec2[i] && char_vec1[i - 1] == char_vec2[i - 1] {
                count = count + 1;
            }
        }

        return char_vec1[0..count].iter().collect();
}
