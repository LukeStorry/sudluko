use super::models::*;

fn serialize_cell(cell: &SudokuCell) -> String {
    match *cell {
        Some(x) => x.to_string(),
        None => ".".to_string(),
    }
}
pub fn serialize(sudoku: Sudoku) -> String {
    sudoku
        .iter()
        .flatten()
        .map(serialize_cell)
        .collect::<Vec<String>>()
        .join("")
}

fn deserialize_cell((cell_index, cell_char): (usize, char)) -> Result<SudokuCell, String> {
    match cell_char {
        '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => {
            Ok(Some(cell_char.to_digit(10).unwrap() as usize))
        }
        '.' => Ok(None),
        _ => Err(format!(
            "Unknown character {} at position {}.",
            cell_char, cell_index
        )),
    }
}
pub fn deserialize(string: String) -> Result<Sudoku, String> {
    if string.len() != 81 {
        return Err(format!(
            "Input string was {} chars long. Expected 81.",
            string.len()
        ));
    }

    let (cells, errors): (Vec<_>, Vec<_>) = string
        .chars()
        .enumerate()
        .map(deserialize_cell)
        .partition(Result::is_ok);

    if !errors.is_empty() {
        return Err(errors
            .iter()
            .map(|e| e.as_ref().err().unwrap().to_string())
            .collect::<Vec<String>>()
            .join(" "));
    }
    let mut sudoku: Sudoku = [[None; 9]; 9];
    for (row_index, row) in cells.chunks(9).enumerate() {
        for (col_index, cell) in row.iter().enumerate() {
            sudoku[row_index][col_index] = *cell.as_ref().unwrap();
        }
    }
    Ok(sudoku)
}

#[cfg(test)]
mod serialization_tests {
    #[test]
    fn serialisation_works() {
        let expected: String = std::iter::repeat("12345689.").take(9).collect();
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
        let result = super::serialize(sudoku);
        assert_eq!(expected, result);
    }
}

#[cfg(test)]
mod deserialization_tests {
    #[test]
    fn rejects_short_string() {
        let input = "99".to_string();
        let expected_error = Err("Input string was 2 chars long. Expected 81.".to_string());
        let result = super::deserialize(input);
        assert_eq!(expected_error, result);
    }
    #[test]
    fn rejects_long_string() {
        let input: String = std::iter::repeat(".").take(82).collect();
        let expected_error = Err("Input string was 82 chars long. Expected 81.".to_string());
        let result = super::deserialize(input);
        assert_eq!(expected_error, result);
    }
    #[test]
    fn handles_bad_chars() {
        let mut input: String = std::iter::repeat("1").take(78).collect();
        input.extend(&['a', '0', '1']);
        let expected_error = Err(
            "Unknown character a at position 78. Unknown character 0 at position 79.".to_string(),
        );
        let result = super::deserialize(input);
        assert_eq!(expected_error, result);
    }
    #[test]
    fn deserialization_works() {
        let input: String = std::iter::repeat("12345689.").take(9).collect();
        let result = super::deserialize(input);
        let expected = [[
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
        assert_eq!(Ok(expected), result);
    }
}
