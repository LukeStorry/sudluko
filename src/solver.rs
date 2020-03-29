use super::models::*;
use rand::seq::SliceRandom;
use rand::thread_rng;

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

pub fn solve(sudoku: Sudoku) -> Option<Sudoku> {
    solve_partial(sudoku, 0)
}

fn solve_partial(previous: Sudoku, cell_index: usize) -> Option<Sudoku> {
    if cell_index == 81 {
        return Some(previous);
    }
    let row = cell_index / 9;
    let column = cell_index % 9;

    if previous[row][column] != None {
        return solve_partial(previous, cell_index + 1);
    }
    let mut sudoku = previous;
    let mut possibilities = [1, 2, 3, 4, 5, 6, 7, 8, 9];
    let mut rng = thread_rng();
    possibilities.shuffle(&mut rng);
    for n in possibilities.iter() {
        sudoku[row][column] = Some(*n);
        if check_valid(sudoku) {
            let next = solve_partial(sudoku, cell_index + 1);
            if next != None {
                return next;
            }
        }
    }
    return None;
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

#[cfg(test)]
mod solve_tests {
    #[test]
    fn from_empty() {
        let sudoku = [[None; 9]; 9];
        let result = super::solve(sudoku);
        assert_eq!(
            405,
            result
                .unwrap()
                .iter()
                .map(|r| r.iter().map(|c| c.unwrap()).sum::<usize>())
                .sum::<usize>()
        );
    }
    #[test]
    fn returns_none_if_unsolvable() {
        let sudoku = [[
            Some(1),
            Some(2),
            Some(3),
            Some(4),
            Some(5),
            Some(6),
            Some(8),
            Some(9),
            None,
        ]; 9];

        let result = super::solve(sudoku);
        assert_eq!(None, result);
    }
    #[test]
    fn solves_correctly() {
        let sudoku =
            ".....2...6.5.8...393...4.8.5.98......26...83......61.5.6.2...514...5.7.6...4....."
                .to_string();
        let solved =
            "178532964645789213932614587519873642726145839384926175863297451491358726257461398"
                .to_string();

        let sudoku = super::super::utils::deserialize(sudoku).unwrap();
        let solved = super::super::utils::deserialize(solved).unwrap();
        let result = super::solve(sudoku);
        assert_eq!(solved, result.unwrap());
    }
}
