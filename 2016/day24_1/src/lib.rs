use thiserror::Error;

#[derive(Debug, PartialEq)]
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

#[cfg(test)]
mod tests {
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
}
