pub fn max_area(height: Vec<i32>) -> i32 {
    let mut max_area = 0;

    for (index1, num1) in height.iter().enumerate() {
        for (index2, num2) in height.iter().enumerate() {
            if index1 != index2 {
                let area_temp = calulate_area(*num1, index1 as i32, *num2, index2 as i32);
                if area_temp > max_area {
                    max_area = area_temp;
                }
            }
        }
    }
    max_area
}

pub fn calulate_area(num1: i32, index1: i32, num2: i32, index2: i32) -> i32 {
    let height = if num1 > num2 { num2 } else { num1 };

    let width = if index1 > index2 {
        index1 - index2
    } else {
        index2 - index1
    };

    height * width
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_with_empty_list() {
        assert_eq!(max_area(Vec::new()), 0);
    }

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
    fn test_with_zero_heigth() {
        let height = vec![0, 0, 0, 0];
        assert_eq!(max_area(height), 0);
    }
}
