use itertools::Itertools;
use ndarray::Array2;
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

#[derive(Debug, PartialEq)]
struct Move {
    from: (usize, usize),
    to: (usize, usize),
}

#[derive(Debug, PartialEq)]
pub struct Grid {
    used: Array2<u32>,
    available: Array2<u32>,
    data: (usize, usize),
}

impl Grid {
    pub fn new(nodes: &[Node]) -> Result<Grid, String> {
        let max_x = match nodes.iter().max_by(|node1, node2| node1.x.cmp(&node2.x)) {
            None => return Err("empty node list".to_owned()),
            Some(node) => node.x,
        };
        let max_y = match nodes.iter().max_by(|node1, node2| node1.y.cmp(&node2.y)) {
            None => return Err("empty node list".to_owned()),
            Some(node) => node.y,
        };
        let mut used = Array2::default((max_y + 1, max_x + 1));
        let mut available = Array2::default((max_y + 1, max_x + 1));
        for y in 0..=max_y {
            for x in 0..=max_x {
                let node = match nodes.iter().find(|node| node.y == y && node.x == x) {
                    Some(found) => found,
                    None => return Err(format!("can't find node y={y} x={x}")),
                };
                used[(y, x)] = node.used;
                available[(y, x)] = node.size - node.used;
            }
        }
        Ok(Grid {
            used,
            available,
            data: (0, max_x),
        })
    }

    fn possible_moves(&self) -> Vec<Move> {
        let mut moves = vec![];
        let dim = self.used.dim();
        let max_y = dim.0 - 1;
        let max_x = dim.1 - 1;
        for y in 0..=max_y {
            for x in 0..=max_x {
                let used = self.used[(y, x)];
                if used == 0 {
                    continue;
                }
                if x > 0 && self.available[(y, x - 1)] >= used {
                    moves.push(Move {
                        from: (y, x),
                        to: (y, x - 1),
                    });
                }
                if y > 0 && self.available[(y - 1, x)] >= used {
                    moves.push(Move {
                        from: (y, x),
                        to: (y - 1, x),
                    });
                }
                if x < max_x && self.available[(y, x + 1)] >= used {
                    moves.push(Move {
                        from: (y, x),
                        to: (y, x + 1),
                    });
                }
                if y < max_y && self.available[(y + 1, x)] >= used {
                    moves.push(Move {
                        from: (y, x),
                        to: (y + 1, x),
                    });
                }
            }
        }
        moves
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
    use ndarray::array;

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

    #[test]
    fn grid_new_test_1() {
        let nodes = [
            Node {
                x: 0,
                y: 0,
                size: 2,
                used: 1,
            },
            Node {
                x: 1,
                y: 0,
                size: 4,
                used: 3,
            },
            Node {
                x: 0,
                y: 1,
                size: 6,
                used: 5,
            },
            Node {
                x: 1,
                y: 1,
                size: 8,
                used: 7,
            },
        ];
        let correct_used = array![[1, 3], [5, 7]];
        let correct_available = array![[1, 1], [1, 1]];
        let correct = Grid {
            used: correct_used,
            available: correct_available,
            data: (0, 1),
        };
        assert_eq!(correct, Grid::new(&nodes).unwrap());
    }

    #[test]
    fn grid_possible_moves_test_1() {
        let used = array![[0, 4, 5], [4, 2, 7]];
        let available = array![[10, 3, 6], [1, 5, 2]];
        let grid = Grid {
            used,
            available,
            data: (0, 2),
        };
        let correct_moves = [
            Move {
                from: (0, 1),
                to: (0, 0),
            },
            Move {
                from: (0, 1),
                to: (0, 2),
            },
            Move {
                from: (0, 1),
                to: (1, 1),
            },
            Move {
                from: (1, 0),
                to: (0, 0),
            },
            Move {
                from: (1, 0),
                to: (1, 1),
            },
            Move {
                from: (1, 1),
                to: (0, 1),
            },
            Move {
                from: (1, 1),
                to: (1, 2),
            },
        ];
        assert_eq!(grid.possible_moves(), correct_moves);
    }
}
