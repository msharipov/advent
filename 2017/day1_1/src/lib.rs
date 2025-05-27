pub fn captcha_solution(captcha: &str) -> Result<i64, &str> {
    let digits: Result<_, _> = captcha
        .chars()
        .map(|c| match c {
            '0'..='9' => Ok(c as u8 - b'0'),
            _ => Err("invalid character"),
        })
        .collect();
    let mut digits: Vec<_> = digits?;
    digits.push(digits[0]);
    let sum = digits
        .windows(2)
        .map(|pair| {
            if pair[0] == pair[1] {
                pair[0] as i64
            } else {
                0i64
            }
        })
        .sum();
    Ok(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn captcha_solution_test_1() {
        let captcha = "84383309921199382122448";
        assert_eq!(captcha_solution(captcha), Ok(36));
    }

    #[test]
    fn captcha_solution_test_2() {
        let captcha = "843833k9921199382122448";
        assert_eq!(captcha_solution(captcha), Err("invalid character"));
    }
}
