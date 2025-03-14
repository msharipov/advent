pub fn get_code_index(row: u64, col: u64) -> u64 {
    let header = row + col - 1;
    (header * header - header) / 2 + col
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn get_code_index_test_1() {
        assert_eq!(get_code_index(3, 4), 19);
    }
}
