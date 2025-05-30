pub fn captcha_solution(captcha: &str) -> Result<i64, &str> {
    let digits: Result<_, _> = captcha
        .chars()
        .map(|c| match c {
            '0'..='9' => Ok(c as u8 - b'0'),
            _ => Err("invalid character"),
        })
        .collect();
    let mut digits: Vec<_> = digits?;
    let len = digits.len();
    digits.append(&mut digits.clone());
    let mut sum = 0;
    for i in 0..len {
        if digits[i] == digits[i + len / 2] {
            sum += digits[i] as i64;
        }
    }
    Ok(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn captcha_solution_test_1() {
        let captcha = "8438330294199382122448";
        assert_eq!(captcha_solution(captcha), Ok(34));
    }

    #[test]
    fn captcha_solution_test_2() {
        let captcha = "843833k9921199382122448";
        assert_eq!(captcha_solution(captcha), Err("invalid character"));
    }
}
