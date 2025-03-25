use sscanf::sscanf;

pub fn supports_tls(ipv7: &str) -> Result<bool, sscanf::Error> {
    let (first, _, last) = sscanf!(ipv7, "{str}[{str}]{str}")?;
    for window in first.chars().collect::<Vec<_>>().windows(4) {
        if window[0] == window[3] && window[1] == window[2] && window[0] != window[1] {
            return Ok(true);
        }
    }
    for window in last.chars().collect::<Vec<_>>().windows(4) {
        if window[0] == window[3] && window[1] == window[2] && window[0] != window[1] {
            return Ok(true);
        }
    }
    Ok(false)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn supports_tls_test_1() {
        assert!(!supports_tls("wwwww[abba]bnmr").unwrap());
    }

    #[test]
    fn supports_tls_test_2() {
        assert!(supports_tls("bbttb[fdgbfg]qwer").unwrap());
    }
}
