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
type SudokuCell = Option<u8>;
type SudokuLine = [SudokuCell; 9];
type SudokuBox = [[SudokuCell; 3]; 3];
type Sudoku = [SudokuLine; 9];

fn get_column(sudoku: Sudoku, colIndex: usize) -> SudokuLine {
    assert!((0..9).contains(&colIndex));
    sudoku.map(|line| line[colIndex])
}

fn get_row(sudoku: Sudoku, rowIndex: usize) -> SudokuLine {
    assert!((0..9).contains(&rowIndex));
    sudoku[rowIndex]
}

fn get_box(sudoku: Sudoku, boxIndex: usize) -> SudokuBox {
    assert!((0..9).contains(&boxIndex));
    let start_row: usize = boxIndex;
    let start_col: usize = boxIndex % 3;
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

#[cfg(test)]
mod tests {
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
        (0..9).for_each(|i| sudoku[i][3] = Some(9 - (i as u8)));
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
        sudoku[3][0] = Some(1);
        sudoku[3][1] = Some(2);
        sudoku[3][2] = Some(3);
        sudoku[4][0] = Some(4);
        sudoku[4][1] = Some(5);
        sudoku[4][2] = Some(6);
        sudoku[5][0] = Some(7);
        sudoku[5][1] = Some(8);
        sudoku[5][2] = Some(9);
        let expected: super::SudokuBox = [
            [Some(1), Some(2), Some(3)],
            [Some(4), Some(5), Some(6)],
            [Some(7), Some(8), Some(9)],
        ];
        let result: super::SudokuBox = super::get_box(sudoku, 3);
        assert_eq!(expected, result);
    }
}
