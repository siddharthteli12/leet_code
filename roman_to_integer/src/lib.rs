pub fn roman_to_int(s: String) -> i32 {
    let string_vec: Vec<char> = s.chars().collect();
    let mut total: i32 = 0;
    let string_length = string_vec.len();
    for i in 0..string_length {
        total += match string_vec[i] {
            'I' => {
                if i < string_length - 1 {
                    match string_vec[i + 1] {
                        'V' | 'X' => -1,
                        _ => 1,
                    }
                } else {
                    1
                }
            }
            'X' => {
                if i < string_length - 1 {
                    match string_vec[i + 1] {
                        'L' | 'C' => -10,
                        _ => 10,
                    }
                } else {
                    10
                }
            }
            'C' => {
                if i < string_length - 1 {
                    match string_vec[i + 1] {
                        'D' | 'M' => -100,

                        _ => 100,
                    }
                } else {
                    100
                }
            }
            'V' => 5,
            'L' => 50,
            'D' => 500,
            'M' => 1000,
            _ => {
                unreachable!();
            }
        };
    }

    total
}
