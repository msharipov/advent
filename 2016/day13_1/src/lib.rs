use std::collections::HashMap;

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
        return self.cells.get(location).unwrap();
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
}
