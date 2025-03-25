pub fn compute_code(door_id: &str) -> String {
    let mut code = String::default();
    let mut index = 0;
    while code.len() < 8 {
        let hash = format!("{:x}", md5::compute(format!("{door_id}{index}")));
        if &hash[0..5] == "00000" {
            code.push(hash.chars().nth(5).unwrap());
        }
        index += 1;
    }
    code
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compute_code_test_1() {
        assert_eq!(compute_code("abc"), "18f47a30");
    }
}
