mod models;
mod solver;
mod utils;

pub use crate::models::Sudoku;
use crate::solver::solve;
use crate::utils::{deserialize, pretty, serialize};

fn generate() -> Sudoku {
    let s: Sudoku = [[None; 9]; 9];
    solve(s).unwrap()
}

fn display(s: Sudoku) -> String {
    format!("{}\n\n{}", serialize(s), pretty(s))
}

pub fn generate_as_string() -> String {
    display(generate())
}

pub fn solve_as_string(sudoku_string: String) -> String {
    let sudoku = deserialize(sudoku_string);
    if sudoku.is_err() {
        return sudoku.err().unwrap().to_string();
    }
    match solve(sudoku.unwrap()) {
        Some(s) => display(s),
        None => "Unsolvable.".to_string(),
    }
}
