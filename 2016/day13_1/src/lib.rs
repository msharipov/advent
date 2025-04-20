use std::collections::{HashMap, HashSet};

#[derive(Clone, Debug, PartialEq)]
pub enum Cell {
    Wall,
    Space,
}

pub struct Maze {
    favorite_num: i64,
    cells: HashMap<(i64, i64), Cell>,
}

impl Maze {
    pub fn new(favorite_num: i64) -> Self {
        Maze {
            favorite_num,
            cells: HashMap::new(),
        }
    }

    pub fn get_cell(&mut self, location: &(i64, i64)) -> &Cell {
        if !self.cells.contains_key(location) {
            let (x, y) = location;
            let n = x * x + 3 * x + 2 * x * y + y + y * y + self.favorite_num;
            let cell = if n.count_ones() % 2 == 0 {
                Cell::Space
            } else {
                Cell::Wall
            };
            self.cells.insert(*location, cell.clone());
        }
        self.cells.get(location).unwrap()
    }

    pub fn shortest_distance(&mut self, from: &(i64, i64), to: &(i64, i64)) -> Option<usize> {
        let mut steps = 0;
        let mut explored = HashSet::new();
        explored.insert(*from);
        let mut horizon = explored.clone();
        while !horizon.contains(to) {
            steps += 1;
            if horizon.is_empty() {
                return None;
            }
            let mut new_horizon = HashSet::new();
            for &(x, y) in horizon.iter() {
                let neighbors = [(x + 1, y), (x - 1, y), (x, y + 1), (x, y - 1)];
                let neighbors: Vec<_> = neighbors
                    .into_iter()
                    .filter(|(new_x, new_y)| {
                        *new_x >= 0 && *new_y >= 0 && !explored.contains(&(*new_x, *new_y))
                    })
                    .collect();
                for neighbor in neighbors {
                    match self.get_cell(&neighbor) {
                        Cell::Space => {
                            new_horizon.insert(neighbor);
                            explored.insert(neighbor);
                        }
                        Cell::Wall => {}
                    }
                }
            }
            horizon = new_horizon;
        }
        Some(steps)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_cell_test_1() {
        let mut maze = Maze::new(10);
        assert_eq!(*maze.get_cell(&(4, 3)), Cell::Wall);
        assert_eq!(*maze.get_cell(&(1, 5)), Cell::Space);
    }

    #[test]
    fn shortest_distance_test_1() {
        let mut maze = Maze::new(10);
        assert_eq!(maze.shortest_distance(&(1, 1), &(7, 4)), Some(11));
    }
}
