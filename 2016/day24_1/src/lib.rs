use std::collections::{HashMap, HashSet};

use itertools::Itertools;
use ndarray::Array2;
use thiserror::Error;

#[derive(Debug, PartialEq, Clone)]
pub enum Tile {
    Floor,
    Wall,
    Marker(u8),
}

#[derive(Debug, PartialEq, Error)]
#[error("invalid character: {c}")]
pub struct ParseTileError {
    c: char,
}

impl TryFrom<char> for Tile {
    type Error = ParseTileError;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '.' => Ok(Tile::Floor),
            '#' => Ok(Tile::Wall),
            '0'..='9' => Ok(Tile::Marker(value as u8 - b'0')),
            _ => Err(ParseTileError { c: value }),
        }
    }
}

type MarkerTable = HashMap<u8, (usize, usize)>;

#[derive(Debug, PartialEq)]
pub struct Map {
    tiles: Array2<Tile>,
    markers: MarkerTable,
}

#[derive(Debug, PartialEq, Error)]
#[error("duplicate markers: {marker}")]
pub struct DuplicateMarkerError {
    marker: u8,
}

#[derive(Debug, PartialEq, Error)]
pub enum ParseMapError {
    #[error("cannot parse tile")]
    TileError(#[from] ParseTileError),
    #[error("line #{line_number} has different length")]
    ShapeError { line_number: usize },
    #[error("duplicate markers")]
    DuplicateMarker(#[from] DuplicateMarkerError),
}

impl Map {
    fn marker_map(tiles: &Array2<Tile>) -> Result<MarkerTable, DuplicateMarkerError> {
        let mut map = MarkerTable::new();
        for (i, tile) in tiles.indexed_iter() {
            if let Tile::Marker(number) = tile {
                if map.contains_key(number) {
                    return Err(DuplicateMarkerError { marker: *number });
                } else {
                    map.insert(*number, i);
                }
            }
        }
        Ok(map)
    }

    pub fn new(tiles: Array2<Tile>) -> Result<Map, DuplicateMarkerError> {
        let markers = Map::marker_map(&tiles)?;
        Ok(Map { tiles, markers })
    }

    pub fn parse_map(lines: &[&str]) -> Result<Map, ParseMapError> {
        if lines.is_empty() {
            return Ok(Map::new(Array2::from_elem((0, 0), Tile::Floor)).unwrap());
        }
        let width = lines[0].len();
        let mut map = vec![];
        for (i, line) in lines.iter().enumerate() {
            if line.len() != width {
                return Err(ParseMapError::ShapeError { line_number: i });
            }
            for c in line.chars() {
                map.push(c.try_into()?);
            }
        }
        let map = Map::new(Array2::from_shape_vec((lines.len(), width), map).unwrap())?;
        Ok(map)
    }

    fn neighbors(&self, (y, x): (usize, usize)) -> Vec<(usize, usize)> {
        let mut valid_neighbors = vec![];
        let shape = self.tiles.shape();
        let y_len = shape[0];
        let x_len = shape[1];
        if y > 0 {
            valid_neighbors.push((y - 1, x));
        }
        if x > 0 {
            valid_neighbors.push((y, x - 1));
        }
        if y_len >= 1 && y < y_len - 1 {
            valid_neighbors.push((y + 1, x));
        }
        if x_len >= 1 && x < x_len - 1 {
            valid_neighbors.push((y, x + 1));
        }
        valid_neighbors
    }

    fn distance(&self, pt1: &(usize, usize), pt2: &(usize, usize)) -> Option<usize> {
        if matches!(self.tiles[*pt1], Tile::Wall) || matches!(self.tiles[*pt2], Tile::Wall) {
            return None;
        }
        let mut step: usize = 0;
        let mut horizon = HashSet::from_iter([*pt1]);
        let mut explored = horizon.clone();
        loop {
            if horizon.is_empty() {
                return None;
            }
            if horizon.contains(&pt2) {
                return Some(step);
            }
            step += 1;
            let mut new_horizon = HashSet::new();
            for tile in horizon {
                let neighbors = self.neighbors(tile);
                for neighbor in neighbors {
                    if !explored.contains(&neighbor) && !matches!(self.tiles[neighbor], Tile::Wall)
                    {
                        new_horizon.insert(neighbor.clone());
                        explored.insert(neighbor);
                    }
                }
            }
            horizon = new_horizon;
        }
    }

    fn distance_map(&self) -> HashMap<(u8, u8), usize> {
        let markers_vec: Vec<_> = self.markers.keys().cloned().collect();
        let mut distances = HashMap::new();
        for pair in markers_vec.iter().permutations(2) {
            let a = pair[0];
            let b = pair[1];
            let pos_a = self.markers.get(a).unwrap();
            let pos_b = self.markers.get(b).unwrap();
            if let Some(dist) = self.distance(pos_a, pos_b) {
                distances.insert((*a, *b), dist);
                distances.insert((*b, *a), dist);
            }
        }
        distances
    }

    fn path_len(&self, path: &[u8], map: HashMap<(u8, u8), usize>) -> Option<usize> {
        if path.is_empty() {
            return None;
        }
        let mut total = 0;
        for leg in path.windows(2) {
            total += map.get(&(leg[0], leg[1]))?;
        }
        Some(total)
    }
}

#[cfg(test)]
mod tests {
    use ndarray::array;

    use super::*;

    #[test]
    fn tile_try_from_test_1() {
        assert_eq!('.'.try_into(), Ok(Tile::Floor));
        assert_eq!('#'.try_into(), Ok(Tile::Wall));
        assert_eq!('3'.try_into(), Ok(Tile::Marker(3)));
    }

    #[test]
    fn tile_try_from_test_2() {
        let result = Tile::try_from('y');
        assert_eq!(result, Err(ParseTileError { c: 'y' }));
    }

    #[test]
    fn parse_map_test_1() {
        use Tile::*;
        let lines = ["#...2", "..3##", "#.#.#"];
        let correct = Map::new(array![
            [Wall, Floor, Floor, Floor, Marker(2)],
            [Floor, Floor, Marker(3), Wall, Wall],
            [Wall, Floor, Wall, Floor, Wall]
        ])
        .unwrap();
        assert_eq!(Map::parse_map(&lines), Ok(correct));
    }

    #[test]
    fn parse_map_test_2() {
        let lines = ["#...2", ".3##", "#.#.#"];
        assert_eq!(
            Map::parse_map(&lines),
            Err(ParseMapError::ShapeError { line_number: 1 })
        );
    }

    #[test]
    fn parse_map_test_3() {
        let lines = ["#...2", "..3##", "#.#2#"];
        assert_eq!(
            Map::parse_map(&lines),
            Err(ParseMapError::DuplicateMarker(DuplicateMarkerError {
                marker: 2
            }))
        );
    }

    #[test]
    fn parse_map_test_4() {
        let lines = ["#...2", "..3##", "#,#.#"];
        assert_eq!(
            Map::parse_map(&lines),
            Err(ParseMapError::TileError(ParseTileError { c: ',' }))
        );
    }

    #[test]
    fn marker_map_test_1() {
        use Tile::*;
        let map = Map::new(array![
            [Wall, Floor, Floor, Floor, Marker(2)],
            [Floor, Floor, Marker(3), Wall, Wall],
            [Wall, Floor, Wall, Floor, Wall]
        ])
        .unwrap();
        let correct = MarkerTable::from_iter([(2, (0, 4)), (3, (1, 2))]);
        assert_eq!(map.markers, correct);
    }

    #[test]
    fn map_distance_test_1() {
        let lines = [".....", ".....", ".....", ".....", "....."];
        let map = Map::parse_map(&lines).unwrap();
        assert_eq!(map.distance(&(0, 1), &(4, 4)), Some(7));
    }

    #[test]
    fn map_distance_test_2() {
        let lines = [".....", ".....", "#####", ".....", "....."];
        let map = Map::parse_map(&lines).unwrap();
        assert_eq!(map.distance(&(0, 1), &(4, 4)), None);
    }

    #[test]
    fn map_distance_test_3() {
        let lines = [".....", ".....", ".####", ".....", "....."];
        let map = Map::parse_map(&lines).unwrap();
        assert_eq!(map.distance(&(0, 3), &(4, 2)), Some(9));
    }

    #[test]
    fn map_distance_test_4() {
        let lines = [".....", ".....", ".####", ".....", "....."];
        let map = Map::parse_map(&lines).unwrap();
        assert_eq!(map.distance(&(0, 3), &(0, 3)), Some(0));
    }

    #[test]
    fn map_distance_test_5() {
        let lines = [".....", ".....", ".####", "####.", "....."];
        let map = Map::parse_map(&lines).unwrap();
        assert_eq!(map.distance(&(3, 2), &(0, 3)), None);
    }

    #[test]
    fn map_distance_test_6() {
        let lines = [".....", ".....", ".####", "####.", "....."];
        let map = Map::parse_map(&lines).unwrap();
        assert_eq!(map.distance(&(0, 0), &(2, 4)), None);
    }

    #[test]
    fn distance_map_test_1() {
        let lines = ["2....", "....4", ".####", ".....", "...7."];
        let map = Map::parse_map(&lines).unwrap();
        let correct = HashMap::from_iter([
            ((2, 4), 5),
            ((4, 2), 5),
            ((2, 7), 7),
            ((7, 2), 7),
            ((4, 7), 10),
            ((7, 4), 10),
        ]);
        assert_eq!(correct, map.distance_map());
    }

    #[test]
    fn path_len_test_1() {
        let lines = [".1..3", "##.##", "...##", ".####", "...5."];
        let map = Map::parse_map(&lines).unwrap();
        let path = [3, 5, 1];
        assert_eq!(map.path_len(&path, map.distance_map()), Some(21));
    }

    #[test]
    fn path_len_test_2() {
        let lines = [".1..3", "##.##", "...##", ".####", "...5."];
        let map = Map::parse_map(&lines).unwrap();
        let path = [];
        assert_eq!(map.path_len(&path, map.distance_map()), None);
    }

    #[test]
    fn path_len_test_3() {
        let lines = [".1..3", "##.##", ".#.##", ".####", "...5."];
        let map = Map::parse_map(&lines).unwrap();
        let path = [3, 5, 1];
        assert_eq!(map.path_len(&path, map.distance_map()), None);
    }
}
