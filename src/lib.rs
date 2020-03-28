mod models;
mod solver;

pub use crate::models::Sudoku;
use crate::solver::solve;

fn generate() -> Sudoku {
    let s: Sudoku = [[None; 9]; 9];
    solve(s)
}
