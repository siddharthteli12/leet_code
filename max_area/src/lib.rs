pub fn max_area(height: Vec<i32>) -> i32 {
    let mut max_area = 0;
    let mut sorted_height: Vec<(usize, i32)> = height
        .iter()
        .enumerate()
        .map(|(index, height)| (index, *height))
        .collect();
    sorted_height.sort_by_key(|(_, key2)| *key2);

    let mut index1 = 0;
    let mut index2 = height.len() - 1;
    let mut num1 = height[index1];
    let (mut act_index, mut num2) = sorted_height[index2];
    max_area = calculate_area(num1, num2, index1, act_index);
    loop {
        let mut index_left = index1 + 1;
        let mut index_right = index2 - 1;
        if index_left == act_index {
            index_left += 1;
        } else if sorted_height[index_right].0 = index1 {
            index_right -= 1;
        }
        let area1 = calculate_area(height[index_left], num2, index_left, act_index);
        let area2 = calculate_area(
            num1,
            sorted_height[index_right].1,
            index_left,
            sorted_height[index_right].0,
        );

        if max_area > area1 && max_area > area2 {
            break;
        } else if area1 > area2 {
            max_area = area1;
            index1 = index_left;
        } else {
            max_area = area2;
            act_index = sorted_height[index_right].1;
            index2 = index_right;
        }
    }

    max_area
}

fn calculate_area(num1: i32, num2: i32, index1: usize, index2: usize) -> i32 {
    let num = if num1 > num2 { num2 } else { num1 };
    let diff = if index1 > index2 {
        index1 - index2
    } else {
        index2 - index1
    };

    num * diff as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn test_with_empty_list() {
    //     assert_eq!(max_area(Vec::new()), 0);
    // }

    #[test]
    fn test_with_simple_list_1() {
        let height = vec![1, 8, 6, 2, 5, 4, 8, 3, 7];
        assert_eq!(max_area(height), 49);
    }
    #[test]
    fn test_with_simple_list_2() {
        let height = vec![3, 1, 4, 7, 1];
        assert_eq!(max_area(height), 9);
    }

    #[test]
    fn test_with_simple_list_3() {
        let height = vec![1, 8, 6, 2, 5, 4, 3, 7, 9];
        assert_eq!(max_area(height), 560);
    }

    #[test]
    fn test_with_simple_list_4() {
        let height = vec![1, 8, 6, 2, 5, 4, 81, 3, 7, 9];
        assert_eq!(max_area(height), 64);
    }

    // #[test]
    // fn test_with_zero_heigth() {
    //     let height = vec![0, 0, 0, 0];
    //     assert_eq!(max_area(height), 0);
    // }
}
