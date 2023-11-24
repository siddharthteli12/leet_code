pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
    matrix
        .iter()
        .flatten()
        .map(|&val| val)
        .collect::<Vec<i32>>()
        .binary_search(&target)
        .is_ok()
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
}
