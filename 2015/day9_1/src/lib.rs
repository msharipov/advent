use sscanf::sscanf;

#[derive(Debug, PartialEq)]
struct Distance {
    from: String,
    to: String,
    dist: u64,
}

impl Distance {
    pub fn new(s: &str) -> Result<Self, &'static str> {
        let parsed = match sscanf!(s, "{String} to {String} = {u64}") {
            Ok(p) => Ok(p),
            Err(_) => Err("cannot parse distance"),
        }?;
        Ok(Distance {
            from: parsed.0,
            to: parsed.1,
            dist: parsed.2,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn distance_new_test_1() {
        assert_eq!(
            Distance::new("Alice to Bob = 42"),
            Ok(Distance {
                from: "Alice".to_owned(),
                to: "Bob".to_string(),
                dist: 42,
            })
        );
    }

    #[test]
    fn distance_new_test_2() {
        assert_eq!(
            Distance::new("Coruscant to Shrek's Cave = Far Far Away"),
            Err("cannot parse distance")
        );
    }
}
