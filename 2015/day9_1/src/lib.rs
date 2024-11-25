use std::{
    cmp::{max, min},
    collections::HashMap,
};

use sscanf::sscanf;

#[derive(Debug, PartialEq)]
struct Distance {
    from: String,
    to: String,
    dist: u64,
}

#[derive(Debug, Default)]
struct GPS {
    map: HashMap<(String, String), u64>,
}

impl Distance {
    pub fn new(s: &str) -> Result<Self, &'static str> {
        let parsed = match sscanf!(s, "{String} to {String} = {u64}") {
            Ok(p) => Ok(p),
            Err(_) => Err("cannot parse distance"),
        }?;
        if parsed.0 == parsed.1 {
            return Err("departure and destination are the same");
        }
        Ok(Distance {
            from: parsed.0,
            to: parsed.1,
            dist: parsed.2,
        })
    }
}

impl GPS {
    fn add_distance(&mut self, dist: &Distance) {
        let Distance {
            from: a,
            to: b,
            dist: separation,
        } = dist;

        if a < b {
            self.map.insert((a.to_owned(), b.to_owned()), *separation);
        } else {
            self.map.insert((b.to_owned(), a.to_owned()), *separation);
        }
    }

    pub fn new(distances: &[&str]) -> Result<Self, &'static str> {
        let mut gps = GPS::default();
        for dist in distances {
            match Distance::new(dist) {
                Ok(d) => gps.add_distance(&d),
                Err(e) => return Err(e),
            }
        }
        Ok(gps)
    }

    pub fn get(&self, points: (&str, &str)) -> Option<&u64> {
        self.map.get(&(
            min(points.0, points.1).to_owned(),
            max(points.0, points.1).to_owned(),
        ))
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

    #[test]
    fn gps_get_test_1() {
        let gps = GPS::new(&["Springfield to Chicago = 202"]).unwrap();
        assert_eq!(gps.get(("Springfield", "Chicago")), Some(&202));
    }

    #[test]
    fn gps_get_test_2() {
        let gps = GPS::new(&["Springfield to Chicago = 202", "Detroit to Chicago = 383"]).unwrap();
        assert_eq!(gps.get(("Springfield", "Detroit")), None);
    }
}
