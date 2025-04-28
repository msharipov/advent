#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Tile {
    Safe,
    Trap,
}

pub fn parse_row(line: &str) -> Result<Vec<Tile>, &str> {
    line.chars()
        .map(|c| match c {
            '^' => Ok(Tile::Trap),
            '.' => Ok(Tile::Safe),
            _ => Err("invalid character"),
        })
        .collect()
}

fn next_row(row: &[Tile]) -> Vec<Tile> {
    let mut new_row = vec![];
    if row.is_empty() {
        return new_row;
    }
    if row.len() == 1 {
        new_row.push(Tile::Safe);
        return new_row;
    }
    new_row.push(row[1]);
    for triple in row.windows(3) {
        new_row.push(if triple[0] != triple[2] {
            Tile::Trap
        } else {
            Tile::Safe
        });
    }
    new_row.push(row[row.len() - 2]);
    new_row
}

type Floor = Vec<Vec<Tile>>;

pub fn generate_floor(row: &[Tile]) -> Floor {
    let mut floor = vec![row.to_vec()];
    while floor.len() < row.len() {
        floor.push(next_row(floor.last().expect("must be nonempty")));
    }
    floor
}

pub fn count_safe(floor: &Floor) -> usize {
    floor
        .iter()
        .map(|row| {
            row.iter()
                .filter(|&tile| matches!(tile, Tile::Safe))
                .count()
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn next_row_test_1() {
        use Tile::{Safe, Trap};
        let row = [Safe];
        assert_eq!(next_row(&row), [Safe]);
        let row = [Trap];
        assert_eq!(next_row(&row), [Safe]);
    }

    #[test]
    fn next_row_test_2() {
        use Tile::{Safe, Trap};
        let row = [Safe, Trap, Trap, Safe, Trap, Trap, Trap];
        assert_eq!(next_row(&row), [Trap, Trap, Trap, Safe, Trap, Safe, Trap]);
    }

    #[test]
    fn generate_floor_test_1() {
        use Tile::{Safe, Trap};
        let row = [Safe, Safe, Trap, Trap];
        let correct = vec![
            vec![Safe, Safe, Trap, Trap],
            vec![Safe, Trap, Trap, Trap],
            vec![Trap, Trap, Safe, Trap],
            vec![Trap, Trap, Safe, Safe],
        ];
        assert_eq!(correct, generate_floor(&row));
    }

    #[test]
    fn count_safe_test_1() {
        use Tile::{Safe, Trap};
        let row = [Safe, Safe, Trap, Trap];
        let floor = generate_floor(&row);
        assert_eq!(count_safe(&floor), 6);
    }

    #[test]
    fn count_safe_test_2() {
        let row = parse_row(".^^.^.^^^^").unwrap();
        let floor = generate_floor(&row);
        assert_eq!(count_safe(&floor), 38);
    }

    #[test]
    fn parse_row_test_1() {
        use Tile::{Safe, Trap};
        let line = "..^.^";
        assert_eq!(parse_row(line).unwrap(), vec![Safe, Safe, Trap, Safe, Trap]);
    }
}
