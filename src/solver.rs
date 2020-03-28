use super::models::*;

fn check_valid_line(line: SudokuLine) -> bool {
    let mut found = [false; 9];
    for cell in line.iter() {
        if let Some(value) = cell {
            if found[*value - 1] {
                return false;
            };
            found[*value - 1] = true;
        }
    }
    true
}

fn check_valid_box(sbox: SudokuBox) -> bool {
    let mut found = [false; 9];
    for cell in sbox.iter().flatten() {
        if let Some(value) = cell {
            if found[*value - 1] {
                return false;
            };
            found[*value - 1] = true;
        }
    }
    true
}

fn check_valid(sudoku: Sudoku) -> bool {
    let columns = (0..9).map(|i| get_column(sudoku, i));
    let rows = (0..9).map(|i| get_row(sudoku, i));
    for line in columns.chain(rows) {
        if !check_valid_line(line) {
            return false;
        }
    }
    let boxes = (0..9).map(|i| get_box(sudoku, i));
    for sbox in boxes {
        if !check_valid_box(sbox) {
            return false;
        }
    }
    true
}

pub fn solve(sudoku: Sudoku) -> Sudoku {
    let mut solved = sudoku;
    solved[5][6] = Some(8); // TODO
    solved
}

#[cfg(test)]
mod validity_tests {
    #[test]
    fn check_valid_line() {
        let mut line: super::SudokuLine = [None; 9];
        assert!(super::check_valid_line(line));
        line[1] = Some(9);
        assert!(super::check_valid_line(line));
        (0..9).for_each(|i| line[i] = Some(9 - i));
        assert!(super::check_valid_line(line));
        line[1] = Some(9);
        assert!(!super::check_valid_line(line));
    }
    #[test]
    fn check_valid_box() {
        let mut sbox: super::SudokuBox = [[None; 3]; 3];
        assert!(super::check_valid_box(sbox));
        sbox[0][1] = Some(9);
        assert!(super::check_valid_box(sbox));
        sbox[2][0] = Some(9);
        assert!(!super::check_valid_box(sbox));
    }
    #[test]
    fn check_valid() {
        let mut sudoku: super::Sudoku = [[None; 9]; 9];
        assert!(super::check_valid(sudoku));
        sudoku[4][7] = Some(9);
        assert!(super::check_valid(sudoku));
        sudoku[3][6] = Some(9);
        assert!(!super::check_valid(sudoku));
        sudoku[3][6] = Some(8);
        assert!(super::check_valid(sudoku));
        sudoku[3][2] = Some(8);
        assert!(!super::check_valid(sudoku));
        sudoku[3][2] = Some(9);
        assert!(super::check_valid(sudoku));
        sudoku[8][2] = Some(9);
        assert!(!super::check_valid(sudoku));
    }
}
