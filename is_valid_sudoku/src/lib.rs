pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
    let mut sudoku_2d = [[0; 9]; 9];

    // Insert values.
    for (row_index, row_values) in board.iter().enumerate() {
        for (column_index, value) in row_values.iter().enumerate() {
            let digit: usize = value.to_digit(10).unwrap_or_default() as usize;
            sudoku_2d[row_index][column_index] = digit;
        }
    }

    // Validate rows.
    for index in 0..9 {
        let mut num_count = [0; 9];
        for index_2 in 0..9 {
            let digit: usize = sudoku_2d[index][index_2];
            if digit != 0 {
                if num_count[digit - 1] == 1 {
                    return false;
                }
                num_count[digit - 1] += 1;
            }
        }
    }

    // Validate cloumns
    for index in 0..9 {
        let mut num_count = [0; 9];
        for index_2 in 0..9 {
            let digit: usize = sudoku_2d[index_2][index];
            if digit != 0 {
                if num_count[digit - 1] == 1 {
                    return false;
                }
                num_count[digit - 1] += 1;
            }
        }
    }

    // Validate box
    for index_row in 0..=2 {
        for index_cloumn in 0..=2 {
            let mut num_count = [0; 9];
            for index_mid in 1..=3 {
                for index_2 in 1..=3 {
                    let row_index = (index_2 + index_row * 3) - 1;
                    let column_index = (index_mid + index_cloumn * 3) - 1;
                    let digit: usize = sudoku_2d[row_index][column_index];
                    if digit != 0 {
                        if num_count[digit - 1] == 1 {
                            return false;
                        }
                        num_count[digit - 1] += 1;
                    }
                }
            }
        }
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_with_board_1() {
        let board = [
            ['5', '3', '.', '.', '7', '.', '.', '.', '.'].to_vec(),
            ['6', '.', '.', '1', '9', '5', '.', '.', '.'].to_vec(),
            ['.', '9', '8', '.', '.', '.', '.', '6', '.'].to_vec(),
            ['8', '.', '.', '.', '6', '.', '.', '.', '3'].to_vec(),
            ['4', '.', '.', '8', '.', '3', '.', '.', '1'].to_vec(),
            ['7', '.', '.', '.', '2', '.', '.', '.', '6'].to_vec(),
            ['.', '6', '.', '.', '.', '.', '2', '8', '.'].to_vec(),
            ['.', '.', '.', '4', '1', '9', '.', '.', '5'].to_vec(),
            ['.', '.', '.', '.', '8', '.', '.', '7', '9'].to_vec(),
        ];
        assert_eq!(is_valid_sudoku(board.to_vec()), true);
    }

    #[test]
    fn test_with_board_2() {
        let board = [
            ['8', '3', '.', '.', '7', '.', '.', '.', '.'].to_vec(),
            ['6', '.', '.', '1', '9', '5', '.', '.', '.'].to_vec(),
            ['.', '9', '8', '.', '.', '.', '.', '6', '.'].to_vec(),
            ['8', '.', '.', '.', '6', '.', '.', '.', '3'].to_vec(),
            ['4', '.', '.', '8', '.', '3', '.', '.', '1'].to_vec(),
            ['7', '.', '.', '.', '2', '.', '.', '.', '6'].to_vec(),
            ['.', '6', '.', '.', '.', '.', '2', '8', '.'].to_vec(),
            ['.', '.', '.', '4', '1', '9', '.', '.', '5'].to_vec(),
            ['.', '.', '.', '.', '8', '.', '.', '7', '9'].to_vec(),
        ];
        assert_eq!(is_valid_sudoku(board.to_vec()), false);
    }

    #[test]
    fn test_with_board_3() {
        let board = [
            ['.', '.', '.', '.', '5', '.', '.', '1', '.'].to_vec(),
            ['.', '4', '.', '3', '.', '.', '.', '.', '.'].to_vec(),
            ['.', '.', '.', '.', '.', '3', '.', '.', '1'].to_vec(),
            ['8', '.', '.', '.', '.', '.', '.', '2', '.'].to_vec(),
            ['.', '.', '2', '.', '7', '.', '.', '.', '.'].to_vec(),
            ['.', '1', '5', '.', '.', '.', '.', '.', '.'].to_vec(),
            ['.', '.', '.', '.', '.', '2', '.', '.', '.'].to_vec(),
            ['.', '2', '.', '9', '.', '.', '.', '.', '.'].to_vec(),
            ['.', '.', '4', '.', '.', '.', '.', '.', '.'].to_vec(),
        ];
        assert_eq!(is_valid_sudoku(board.to_vec()), false);
    }

    #[test]
    fn test_with_board_4() {
        let board = [
            ['.', '.', '.', '.', '.', '.', '5', '.', '.'].to_vec(),
            ['.', '.', '.', '.', '.', '.', '.', '.', '.'].to_vec(),
            ['.', '.', '.', '.', '.', '.', '.', '.', '.'].to_vec(),
            ['9', '3', '.', '.', '2', '.', '4', '.', '.'].to_vec(),
            ['.', '.', '7', '.', '.', '.', '3', '.', '.'].to_vec(),
            ['.', '.', '.', '.', '.', '.', '.', '.', '.'].to_vec(),
            ['.', '.', '.', '3', '4', '.', '.', '.', '.'].to_vec(),
            ['.', '.', '.', '.', '.', '3', '.', '.', '.'].to_vec(),
            ['.', '.', '.', '.', '.', '5', '2', '.', '.'].to_vec(),
        ];
        assert_eq!(is_valid_sudoku(board.to_vec()), false);
    }
}
