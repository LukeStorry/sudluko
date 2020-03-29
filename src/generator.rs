use super::models::*;
use super::solver::solve;
use rand::Rng;

pub fn generate() -> Sudoku {
    let mut rng = rand::thread_rng();
    let mut s: Sudoku = solve([[None; 9]; 9]).unwrap();
    loop {
        let previous = s.clone();
        let random = rng.gen_range(0, 81);
        s[random / 9][random % 9] = None;
        if !unique(s) {
            return previous;
        }
    }
}

fn unique(s: Sudoku) -> bool {
    let solution = solve(s).unwrap();
    for _ in 0..5 {
        if solve(s).unwrap() != solution {
            return false;
        }
    }
    return true;
}

#[cfg(test)]
mod uniqueness_tests {
    #[test]
    fn can_succeed() {
        let sudoku =
            ".....2...6.5.8...393...4.8.5.98......26...83......61.5.6.2...514...5.7.6...4....."
                .to_string();
        let sudoku = super::super::utils::deserialize(sudoku).unwrap();
        let result = super::unique(sudoku);
        assert!(result);
    }
    #[test]
    fn fails_on_harder() {
        let sudoku =
            ".....2...6.5.8...3.....4.8.5.98......26...83......61.5.6.2...514...5.7.6...4....."
                .to_string();
        let sudoku = super::super::utils::deserialize(sudoku).unwrap();
        let result = super::unique(sudoku);
        assert!(!result);
    }
    #[test]
    fn fails_on_empty() {
        let result = super::unique([[None; 9]; 9]);
        assert!(!result);
    }
}
#[cfg(test)]
mod generator_tests {
    #[test]
    fn can_succeed() {
        super::generate();
    }
}
