pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
    let cloums_length = matrix[0].len();
    let rows_length = matrix.len();

    let mut left_index = 0;
    let mut right_index = (rows_length * cloums_length) - 1;

    while right_index >= left_index {
        let mid_index = (right_index + left_index) / 2;
        let row_index = mid_index / cloums_length;
        let cloums_index = mid_index % cloums_length;

        if matrix[row_index][cloums_index] > target {
            if let Some(new_index) = mid_index.checked_sub(1) {
                right_index = new_index;
            } else {
                break;
            }
        } else if matrix[row_index][cloums_index] < target {
            left_index = mid_index + 1;
        } else {
            return true;
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let matrix = vec![vec![1, 3, 5, 7], vec![10, 11, 16, 20], vec![23, 30, 34, 60]];
        let target = 3;
        assert_eq!(search_matrix(matrix, target), true);
    }

    #[test]
    fn it_works_1() {
        let matrix = vec![vec![1, 3, 5, 7], vec![10, 11, 16, 20], vec![23, 30, 34, 60]];
        let target = 13;
        assert_eq!(search_matrix(matrix, target), false);
    }

    #[test]
    fn it_works_2() {
        let matrix = vec![vec![1]];
        let target = 0;
        assert_eq!(search_matrix(matrix, target), false);
    }
}
