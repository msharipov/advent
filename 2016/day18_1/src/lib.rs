#[derive(Debug, PartialEq, Clone, Copy)]
enum Tile {
    Safe,
    Trap,
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
}
