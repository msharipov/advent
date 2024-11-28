pub fn increment_password(pass: &str) -> String {
    let mut inc = true;
    let mut new: Vec<_> = pass.bytes().rev().map(|c| {
        if inc {
            if c == b'z' {
                return b'a'
            } else {
                inc = false;
                return c + 1
            }
        }
        c
    }).collect();
    new.reverse();
    String::from_utf8(new).unwrap()
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn increment_password_test_1() {
        assert_eq!(increment_password("vneorvzzz"), "vneorwaaa");
    }
}
