use std::collections::HashMap;

use sscanf::sscanf;

// value => initial bot
type Initial = (u32, u32);
// number of the bot => (low, high)
type Transfers = HashMap<u32, (u32, u32)>;

pub fn parse_instructions(lines: &[&str]) -> Result<(Vec<Initial>, Transfers), sscanf::Error> {
    let mut transfers = Transfers::new();
    let mut initials = vec![];
    for line in lines {
        if let Ok(init) = sscanf!(line, "value {u32} goes to bot {u32}") {
            initials.push(init);
            continue;
        }
        if let Ok((bot, low, high)) = sscanf!(
            line,
            "bot {u32} gives low to bot {u32} and high to bot {u32}"
        ) {
            transfers.insert(bot, (low, high));
            continue;
        }
        return Err(sscanf::Error::MatchFailed);
    }
    Ok((initials, transfers))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_instructions_test_1() {
        let lines = [
            "bot 18 gives low to bot 10 and high to bot 13",
            "value 7 goes to bot 19",
            "bot 7 gives low to bot 34 and high to bot 16",
            "bot 9 gives low to bot 50 and high to bot 10",
            "value 10 goes to bot 5",
        ];
        let transfers = Transfers::from_iter([(18, (10, 13)), (7, (34, 16)), (9, (50, 10))]);
        let initials = vec![(7, 19), (10, 5)];
        assert_eq!(parse_instructions(&lines).unwrap(), (initials, transfers));
    }

    #[test]
    fn parse_instructions_test_2() {
        let lines = [
            "bot 18 gives high to bot 10 and low to bot 13",
            "value 7 goes to bot 19",
            "bot 7 gives low to bot 34 and high to bot 16",
            "bot 9 gives low to bot 50 and high to bot 10",
            "value 10 goes to bot 5",
        ];
        assert!(parse_instructions(&lines).is_err());
    }
}
