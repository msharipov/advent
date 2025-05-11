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
            '0'..='9' => Ok(Tile::Marker(value as u8 - '0' as u8)),
            _ => Err(ParseTileError { c: value }),
        }
    }
}

#[derive(Debug, PartialEq, Error)]
pub enum ParseMapError {
    #[error("cannot parse tile")]
    TileError(#[from] ParseTileError),
    #[error("line #{line_number} has different length")]
    ShapeError { line_number: usize },
}

pub fn parse_map(lines: &[&str]) -> Result<Array2<Tile>, ParseMapError> {
    if lines.is_empty() {
        return Ok(Array2::from_elem((0, 0), Tile::Floor));
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
    Ok(Array2::from_shape_vec((lines.len(), width), map).unwrap())
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
        let correct = array![
            [Wall, Floor, Floor, Floor, Marker(2)],
            [Floor, Floor, Marker(3), Wall, Wall],
            [Wall, Floor, Wall, Floor, Wall]
        ];
        assert_eq!(parse_map(&lines), Ok(correct));
    }

    #[test]
    fn parse_map_test_2() {
        let lines = ["#...2", ".3##", "#.#.#"];
        assert_eq!(
            parse_map(&lines),
            Err(ParseMapError::ShapeError { line_number: 1 })
        );
    }

    #[test]
    fn parse_map_test_3() {
        let lines = ["#...2", "..3##", "#,#.#"];
        assert_eq!(
            parse_map(&lines),
            Err(ParseMapError::TileError(ParseTileError { c: ',' }))
        );
    }
}
