use sscanf::sscanf;

pub fn supports_tls(ipv7: &str) -> Result<bool, sscanf::Error> {
    let mut parts = vec![];
    let mut in_brackets = vec![];
    let mut current = ipv7;
    while sscanf!(current, "{str}[{str}]{str}").is_ok() {
        let (first, middle, last) = sscanf!(current, "{str}[{str}]{str}")?;
        parts.push(first.to_owned());
        in_brackets.push(middle.to_owned());
        current = last;
    }
    parts.push(current.to_owned());
    for part in in_brackets {
        for window in part.chars().collect::<Vec<_>>().windows(4) {
            if window[0] == window[3] && window[1] == window[2] && window[0] != window[1] {
                return Ok(false);
            }
        }
    }
    for part in parts {
        for window in part.chars().collect::<Vec<_>>().windows(4) {
            if window[0] == window[3] && window[1] == window[2] && window[0] != window[1] {
                return Ok(true);
            }
        }
    }
    Ok(false)
}

pub fn supports_ssl(ipv7: &str) -> Result<bool, sscanf::Error> {
    let mut parts = vec![];
    let mut in_brackets = vec![];
    let mut current = ipv7;
    while sscanf!(current, "{str}[{str}]{str}").is_ok() {
        let (first, middle, last) = sscanf!(current, "{str}[{str}]{str}")?;
        parts.push(first.to_owned());
        in_brackets.push(middle.to_owned());
        current = last;
    }
    parts.push(current.to_owned());

    let mut abas = vec![];
    for part in parts {
        for window in part.chars().collect::<Vec<_>>().windows(3) {
            if window[0] == window[2] && window[0] != window[1] {
                abas.push(window.to_owned());
            }
        }
    }
    for part in in_brackets {
        for window in part.chars().collect::<Vec<_>>().windows(3) {
            if window[0] == window[2] && window[0] != window[1] {
                if abas.contains(&vec![window[1], window[0], window[1]]) {
                    return Ok(true);
                }
            }
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

    #[test]
    fn supports_tls_test_3() {
        assert!(supports_tls("verta[sdff]eerre[nmredx]adewet").unwrap());
    }

    #[test]
    fn supports_ssl_test_1() {
        assert!(supports_ssl("asbdfg[trkr]sadsf[weqb]krka").unwrap());
    }

    #[test]
    fn supports_ssl_test_2() {
        assert!(!supports_ssl("asbdfg[trkr]sadsf[wkrk]speda").unwrap());
    }
}
