use std::{collections::HashMap, vec};

pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
    let mut rows = vec![0_u32; 9];

    // Valid rows
    for (row_index, row_values) in board.iter().enumerate() {
        for (value) in row_values.iter() {
            let digit: usize = value.to_digit(10).unwrap() as usize;
            if rows[digit] > 0 {
                return false; 
            } else {
                rows[digit] += 1;
            }
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {}
}
