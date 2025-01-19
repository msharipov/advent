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
}
