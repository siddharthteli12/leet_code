use std::cmp;

pub fn max_area(height: Vec<i32>) -> i32 {
    let mut max_area = 0;
    let mut left = 0;
    let mut right = height.len() - 1;

    while left < right {
        let area = cmp::min(height[left], height[right]) * (right - left) as i32;
        if area > max_area {
            max_area = area;
        }
        if height[left] > height[right] {
            right -= 1;
        } else {
            left += 1;
        }
    }
    max_area
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
        assert_eq!(max_area(height), 56);
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
