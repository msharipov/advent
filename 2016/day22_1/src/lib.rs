use sscanf::sscanf;
use std::str::FromStr;

#[derive(Debug, PartialEq)]
pub struct Node {
    x: usize,
    y: usize,
    size: u32,
    used: u32,
}

impl FromStr for Node {
    type Err = sscanf::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (x, y, _, size, _, used, _) = sscanf!(
            s,
            "/dev/grid/node-x{usize}-y{usize}{str:/[ ]+/}{u32}T{str:/[ ]+/}{u32}T{str}",
        )?;
        Ok(Node { x, y, size, used })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn node_from_str_test_1() {
        let node_str = "/dev/grid/node-x13-y24   112T   62T    50T   55%";
        assert_eq!(
            node_str.parse::<Node>().unwrap(),
            Node {
                x: 13,
                y: 24,
                size: 112,
                used: 62
            }
        );
    }
}
