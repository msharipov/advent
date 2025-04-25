use itertools::Itertools;

fn next_iter(initial: &str) -> String {
    let b: String = initial
        .chars()
        .rev()
        .map(|c| if c == '0' { '1' } else { '0' })
        .collect();
    format!("{initial}0{b}")
}

fn generate_data(initial: &str, size: usize) -> String {
    let mut current = initial.to_owned();
    while current.len() < size {
        current = next_iter(&current);
    }
    current.chars().take(size).collect()
}

fn checksum(data: &str) -> String {
    let mut data = data.to_owned();
    while data.len() % 2 == 0 {
        data = data
            .chars()
            .tuple_windows()
            .step_by(2)
            .flat_map(
                |(first, second)| {
                    if first == second { Some(first) } else { None }
                },
            )
            .collect();
    }
    data
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn next_iter_test_1() {
        assert_eq!(next_iter("1010"), "101001010");
    }

    #[test]
    fn generate_data_test_1() {
        assert_eq!(generate_data("10", 11), "10010010110")
    }

    #[test]
    fn generate_data_test_2() {
        assert_eq!(generate_data("10", 15), "100100101100100")
    }

    #[test]
    fn checksum_test_1() {
        assert_eq!(checksum(&generate_data("10", 16)), "010");
    }
}
