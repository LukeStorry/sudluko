trait Mappable<X, Y, T> {
    fn map<F: Fn(&X) -> Y>(&self, f: F) -> T;
}

impl<U, V> Mappable<U, V, [V; 9]> for [U; 9] {
    fn map<F: Fn(&U) -> V>(&self, f: F) -> [V; 9] {
        [
            f(&self[0]),
            f(&self[1]),
            f(&self[2]),
            f(&self[3]),
            f(&self[4]),
            f(&self[5]),
            f(&self[6]),
            f(&self[7]),
            f(&self[8]),
        ]
    }
}
type SudokuCell = Option<usize>;
type SudokuLine = [SudokuCell; 9];
type SudokuBox = [[SudokuCell; 3]; 3];
type Sudoku = [SudokuLine; 9];

fn get_column(sudoku: Sudoku, col_index: usize) -> SudokuLine {
    assert!((0..9).contains(&col_index));
    sudoku.map(|line| line[col_index])
}

fn get_row(sudoku: Sudoku, row_index: usize) -> SudokuLine {
    assert!((0..9).contains(&row_index));
    sudoku[row_index]
}

fn get_box(sudoku: Sudoku, box_index: usize) -> SudokuBox {
    assert!((0..9).contains(&box_index));
    let start_row: usize = (box_index / 3) * 3;
    let start_col: usize = (box_index % 3) * 3;
    [
        [
            sudoku[start_row][start_col],
            sudoku[start_row][start_col + 1],
            sudoku[start_row][start_col + 2],
        ],
        [
            sudoku[start_row + 1][start_col],
            sudoku[start_row + 1][start_col + 1],
            sudoku[start_row + 1][start_col + 2],
        ],
        [
            sudoku[start_row + 2][start_col],
            sudoku[start_row + 2][start_col + 1],
            sudoku[start_row + 2][start_col + 2],
        ],
    ]
}

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

#[cfg(test)]
mod access {
    #[test]
    fn can_create_sudoku_object() {
        let mut sudoku: super::Sudoku = [[Some(1); 9]; 9];
        sudoku[0][0] = None;
        assert_eq!(sudoku, sudoku);
        assert_eq!(sudoku[0][0], None);
        assert_eq!(sudoku[0][1], Some(1));
    }
    #[test]
    fn get_column() {
        let mut sudoku: super::Sudoku = [[None; 9]; 9];
        (0..9).for_each(|i| sudoku[i][3] = Some(9 - i));
        let expected: super::SudokuLine = [
            Some(9),
            Some(8),
            Some(7),
            Some(6),
            Some(5),
            Some(4),
            Some(3),
            Some(2),
            Some(1),
        ];
        let result: super::SudokuLine = super::get_column(sudoku, 3);
        assert_eq!(expected, result);
    }
    #[test]
    fn get_row() {
        let mut sudoku: super::Sudoku = [[None; 9]; 9];
        let expected: super::SudokuLine = [Some(1); 9];
        sudoku[5] = expected;
        let result: super::SudokuLine = super::get_row(sudoku, 5);
        assert_eq!(expected, result);
    }
    #[test]
    fn get_box() {
        let mut sudoku: super::Sudoku = [[None; 9]; 9];
        sudoku[6][3] = Some(1);
        sudoku[6][4] = Some(2);
        sudoku[6][5] = Some(3);
        sudoku[7][3] = Some(4);
        sudoku[7][4] = Some(5);
        sudoku[7][5] = Some(6);
        sudoku[8][3] = Some(7);
        sudoku[8][4] = Some(8);
        sudoku[8][5] = Some(9);
        let expected: super::SudokuBox = [
            [Some(1), Some(2), Some(3)],
            [Some(4), Some(5), Some(6)],
            [Some(7), Some(8), Some(9)],
        ];
        let result: super::SudokuBox = super::get_box(sudoku, 7);
        assert_eq!(expected, result);
    }
}

#[cfg(test)]
mod validity {
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
