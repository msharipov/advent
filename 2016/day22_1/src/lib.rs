use itertools::Itertools;
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

pub fn count_viable_pairs(nodes: &[Node]) -> usize {
    nodes
        .iter()
        .permutations(2)
        .filter(|perm| perm[0].used != 0 && perm[0].used < (perm[1].size - perm[1].used))
        .count()
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

    #[test]
    fn count_viable_pairs_test_1() {
        let nodes = [
            Node {
                x: 11,
                y: 8,
                size: 25,
                used: 0,
            },
            Node {
                x: 2,
                y: 26,
                size: 14,
                used: 11,
            },
            Node {
                x: 10,
                y: 3,
                size: 21,
                used: 6,
            },
        ];
        assert_eq!(count_viable_pairs(&nodes), 3);
    }
}
