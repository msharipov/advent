use itertools::Itertools;
use ndarray::{Array2, ShapeError};

pub fn parse_matrix(lines: &[&str]) -> Result<Array2<bool>, ShapeError> {
    let dim = lines.len();
    let vec = lines
        .iter()
        .map(|line| {
            line.chars().map(|c| match c {
                '#' => true,
                '.' => false,
                _ => panic!("invalid character"),
            })
        })
        .flatten()
        .collect::<Vec<_>>();
    let mat = Array2::from_shape_vec([dim, dim], vec);
    if let Err(e) = mat {
        return Err(e);
    }
    let mut mat = mat.unwrap();
    mat[(0, 0)] = true;
    mat[(0, dim - 1)] = true;
    mat[(dim - 1, 0)] = true;
    mat[(dim - 1, dim - 1)] = true;
    Ok(mat)
}

pub fn next_step(initial: &Array2<bool>) -> Array2<bool> {
    let dim = initial.dim();
    let (rows, cols) = dim;
    let mut next = Array2::default(initial.raw_dim());
    for (row, col) in (0..rows).cartesian_product(0..cols) {
        let neighbors = (row as i64 - 1..=row as i64 + 1)
            .cartesian_product(col as i64 - 1..=col as i64 + 1)
            .map(|(r, c)| {
                if (row as i64 == r && col as i64 == c)
                    || r < 0
                    || c < 0
                    || r >= rows as i64
                    || c >= cols as i64
                {
                    return 0;
                }
                if initial[(r as usize, c as usize)] {
                    1
                } else {
                    0
                }
            })
            .sum::<u64>();
        let already_on = initial[(row, col)];
        if neighbors == 3 || (already_on && neighbors == 2) {
            next[(row, col)] = true;
        }
    }
    next[(0, 0)] = true;
    next[(0, cols - 1)] = true;
    next[(rows - 1, 0)] = true;
    next[(rows - 1, cols - 1)] = true;
    next
}

pub fn count_on(matrix: &Array2<bool>) -> u64 {
    matrix
        .iter()
        .map(|cell| match cell {
            true => 1,
            false => 0,
        })
        .sum::<u64>()
}

#[cfg(test)]
mod tests {
    use ndarray::arr2;

    use super::*;

    #[test]
    fn parse_matrix_test_1() {
        let lines = ["..#", ".#.", "#.."];
        let mat = parse_matrix(&lines).unwrap();
        let correct = arr2(&[
            [true, false, true],
            [false, true, false],
            [true, false, true],
        ]);
        assert_eq!(mat, correct);
    }

    #[test]
    fn next_step_test_1() {
        let initial = parse_matrix(&["..#", ".#.", "#.."]).unwrap();
        let next = next_step(&initial);
        let correct = parse_matrix(&[".#.", "#.#", ".#."]).unwrap();
        assert_eq!(next, correct);
    }

    #[test]
    fn next_step_test_2() {
        let initial =
            parse_matrix(&["##.#.#", "...##.", "#....#", "..#...", "#.#..#", "####.#"]).unwrap();
        let next = next_step(&initial);
        let correct =
            parse_matrix(&["#.##.#", "####.#", "...##.", "......", "#...#.", "#.####"]).unwrap();
        assert_eq!(next, correct);
    }

    #[test]
    fn count_on_test_1() {
        let matrix =
            parse_matrix(&[".#.#.#", "...##.", "#....#", "..#...", "#.#..#", "####.."]).unwrap();
        assert_eq!(count_on(&matrix), 17)
    }
}
