use itertools::Itertools;

pub fn decode_message(lines: &[&str]) -> String {
    let len = lines[0].chars().count();
    let mut letters = vec![vec![]; len];
    for line in lines {
        for (i, c) in line.chars().enumerate() {
            letters[i].push(c);
        }
    }
    let mut message = String::default();
    for letter in letters {
        let counts = letter.into_iter().counts();
        let most_common = counts
            .iter()
            .sorted_by(|a, b| a.1.cmp(b.1))
            .nth_back(0)
            .unwrap()
            .0;
        message.push(*most_common);
    }
    message
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decode_message_test_1() {
        let code = ["havlv", "aeele", "hnbfo", "nelqo", "relba", "htpmo"];
        assert_eq!(decode_message(&code), "hello");
    }
}
