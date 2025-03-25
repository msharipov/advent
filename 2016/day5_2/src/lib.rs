pub fn compute_code(door_id: &str) -> String {
    let mut index = 0;
    let mut status = [false; 8];
    let mut code = ['_'; 8];
    while status != [true; 8] {
        let hash = format!("{:x}", md5::compute(format!("{door_id}{index}")));
        if &hash[0..5] == "00000" {
            let position = (hash.chars().nth(5).unwrap() as i32 - '0' as i32) as usize;
            if position <= 7 && !status[position] {
                status[position] = true;
                code[position] = hash.chars().nth(6).unwrap();
                println!("{}", String::from_iter(code));
            }
        }
        index += 1;
    }
    String::from_iter(code)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compute_code_test_1() {
        assert_eq!(compute_code("abc"), "05ace8e3");
    }
}
