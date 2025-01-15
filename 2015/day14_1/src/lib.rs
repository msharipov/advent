use sscanf::sscanf;

#[derive(Debug, PartialEq)]
pub struct Reindeer {
    name: String,
    speed: i32,
    duration: i32,
    rest: i32,
}

impl Reindeer {
    pub fn new(line: &str) -> Result<Self, sscanf::Error> {
        let parsed =
            sscanf!(line,
            "{String} can fly {i32} km/s for {i32} seconds, but then must rest for {i32} seconds."
        )?;
        Ok(Reindeer {
            name: parsed.0,
            speed: parsed.1,
            duration: parsed.2,
            rest: parsed.3,
        })
    }

    pub fn position(&self, time: i32) -> f32 {
        let whole_period = self.duration + self.rest;
        let distance_per_period = (self.speed * self.duration) as f32;
        let mut distance = distance_per_period * (time / whole_period) as f32;
        let remaining_time = time % whole_period;
        if remaining_time > self.duration {
            distance += distance_per_period;
        } else {
            distance += distance_per_period * (remaining_time as f32 / self.duration as f32);
        }
        distance
    }
}

fn parse_reindeer(lines: &[&str]) -> Result<Vec<Reindeer>, sscanf::Error> {
    lines.iter().map(|line| Reindeer::new(line)).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reindeer_new_test_1() {
        assert_eq!(
            Reindeer::new(
                "Rudolph can fly 123 km/s for 5 seconds, but then must rest for 10 seconds."
            )
            .unwrap(),
            Reindeer {
                name: "Rudolph".to_string(),
                speed: 123,
                duration: 5,
                rest: 10,
            }
        )
    }

    #[test]
    fn reindeer_new_test_2() {
        assert!(Reindeer::new(
            "Rudolph can fly 12.3 km/s for 5 seconds, but then must rest for 10 seconds."
        )
        .is_err())
    }

    #[test]
    fn reindeer_position_test_1() {
        let reindeer = Reindeer {
            name: "Rudolph".to_owned(),
            speed: 10,
            duration: 4,
            rest: 12,
        };
        assert_eq!(reindeer.position(34), 100.0);
    }
}
