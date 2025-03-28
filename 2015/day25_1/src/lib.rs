use sscanf::sscanf;

pub fn get_code_index(row: u64, col: u64) -> u64 {
    let header = row + col - 1;
    (header * header - header) / 2 + col
}

pub fn compute_code(row: u64, col: u64, seed: u64) -> u64 {
    let index = get_code_index(row, col);
    let mut code = seed;
    for _ in 1..index {
        code = (252533 * code) % 33554393;
    }
    code
}

pub fn parse_input(line: &str) -> Result<(u64, u64), sscanf::Error> {
    sscanf!(line, "To continue, please consult the code grid in the manual.  Enter the code at row {u64}, column {u64}.")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn get_code_index_test_1() {
        assert_eq!(get_code_index(3, 4), 19);
    }

    #[test]
    pub fn compute_code_test_1() {
        assert_eq!(compute_code(2, 6, 20151125), 4041754);
    }

    #[test]
    pub fn parse_input_test_1() {
        let line = "To continue, please consult the code grid in the manual.  Enter the code at row 123, column 456.";
        assert_eq!(parse_input(line).unwrap(), (123, 456));
    }
}
