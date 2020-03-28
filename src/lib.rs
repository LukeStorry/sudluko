mod models;
mod solver;
mod utils;

pub use crate::models::Sudoku;
use crate::solver::solve;
use crate::utils::serialize;

fn generate() -> Sudoku {
    let s: Sudoku = [[None; 9]; 9];
    solve(s)
}

pub fn generate_as_string() -> String {
    serialize(generate())
}
