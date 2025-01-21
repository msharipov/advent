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
    mat
}

pub fn next_step(initial: &Array2<bool>) -> Array2<bool> {
    let dim = initial.dim();
    let (rows, cols) = dim;
    let mut next = Array2::default(initial.raw_dim());
    for row in 0..rows {
        for col in 0..cols {
            let mut neighbors = 0;
            if row > 0 {
                if initial[(row - 1, col)] {
                    neighbors += 1;
                }
            }
            if row > 0 && col > 0 {
                if initial[(row - 1, col - 1)] {
                    neighbors += 1;
                }
            }
            if row > 0 && col + 1 < cols {
                if initial[(row - 1, col + 1)] {
                    neighbors += 1;
                }
            }
            if col > 0 {
                if initial[(row, col - 1)] {
                    neighbors += 1;
                }
            }
            if col + 1 < cols {
                if initial[(row, col + 1)] {
                    neighbors += 1;
                }
            }
            if row + 1 < rows {
                if initial[(row + 1, col)] {
                    neighbors += 1;
                }
            }
            if row + 1 < rows && col > 0 {
                if initial[(row + 1, col - 1)] {
                    neighbors += 1;
                }
            }
            if row + 1 < rows && col + 1 < cols {
                if initial[(row + 1, col + 1)] {
                    neighbors += 1;
                }
            }
            let already_on = initial[(row, col)];
            if neighbors == 3 || (already_on && neighbors == 2) {
                next[(row, col)] = true;
            }
        }
    }
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
            [false, false, true],
            [false, true, false],
            [true, false, false],
        ]);
        assert_eq!(mat, correct);
    }

    #[test]
    fn next_step_test_1() {
        let initial = parse_matrix(&["..#", ".#.", "#.."]).unwrap();
        let next = next_step(&initial);
        let correct = parse_matrix(&["...", ".#.", "..."]).unwrap();
        assert_eq!(next, correct);
    }

    #[test]
    fn next_step_test_2() {
        let initial =
            parse_matrix(&[".#.#.#", "...##.", "#....#", "..#...", "#.#..#", "####.."]).unwrap();
        let next = next_step(&initial);
        let correct =
            parse_matrix(&["..##..", "..##.#", "...##.", "......", "#.....", "#.##.."]).unwrap();
        assert_eq!(next, correct);
    }

    #[test]
    fn count_on_test_1() {
        let matrix =
            parse_matrix(&[".#.#.#", "...##.", "#....#", "..#...", "#.#..#", "####.."]).unwrap();
        assert_eq!(count_on(&matrix), 15);
    }
}
