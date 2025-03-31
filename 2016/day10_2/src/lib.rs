use std::collections::HashMap;

use sscanf::sscanf;

// value => initial bot
type Initial = (u32, u32);

#[derive(Debug, PartialEq, Hash, Eq, Clone)]
pub enum Destination {
    Bot(u32),
    Output(u32),
}

// number of the bot => (low, high)
type Transfers = HashMap<u32, (Destination, Destination)>;

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
            transfers.insert(bot, (Destination::Bot(low), Destination::Bot(high)));
            continue;
        }
        if let Ok((bot, low, high)) = sscanf!(
            line,
            "bot {u32} gives low to output {u32} and high to bot {u32}"
        ) {
            transfers.insert(bot, (Destination::Output(low), Destination::Bot(high)));
            continue;
        }
        if let Ok((bot, low, high)) = sscanf!(
            line,
            "bot {u32} gives low to bot {u32} and high to output {u32}"
        ) {
            transfers.insert(bot, (Destination::Bot(low), Destination::Output(high)));
            continue;
        }
        if let Ok((bot, low, high)) = sscanf!(
            line,
            "bot {u32} gives low to output {u32} and high to output {u32}"
        ) {
            transfers.insert(bot, (Destination::Output(low), Destination::Output(high)));
            continue;
        }
        return Err(sscanf::Error::MatchFailed);
    }
    Ok((initials, transfers))
}

#[derive(Debug, PartialEq, Default)]
pub struct Bot {
    numbers: Vec<u32>,
}

impl Bot {
    fn has_two(&self) -> bool {
        self.numbers.len() == 2
    }

    fn numbers(&self) -> &[u32] {
        self.numbers.as_slice()
    }

    fn give(&mut self, num: u32) {
        self.numbers.push(num);
        self.numbers.sort();
    }
}

type Bots = HashMap<u32, Bot>;

#[derive(Default, Debug, PartialEq)]
pub struct State {
    bots: Bots,
    outputs: HashMap<u32, u32>,
}

pub fn set_up_bots(initial: &[Initial]) -> State {
    let mut bots = HashMap::new();
    for instruction in initial {
        let &(value, bot) = instruction;
        match bots.get_mut(&bot) {
            None => {
                bots.insert(
                    bot,
                    Bot {
                        numbers: vec![value],
                    },
                );
            }
            Some(b) => {
                b.give(value);
            }
        }
    }
    State {
        bots,
        ..State::default()
    }
}

pub fn next_state(current: &State, transfers: &Transfers) -> State {
    let mut new_bots: Bots = HashMap::new();
    let mut outputs = current.outputs.clone();
    for (i, bot) in &current.bots {
        let (low, high) = transfers
            .get(&i)
            .expect("missing transfer instruction")
            .to_owned();
        if bot.has_two() {
            if !new_bots.contains_key(&i) {
                new_bots.insert(*i, Bot::default());
            }
            let numbers = bot.numbers();
            match low {
                Destination::Bot(low_bot) => match new_bots.get_mut(&low_bot) {
                    Some(new_bot) => {
                        new_bot.give(numbers[0]);
                    }
                    None => {
                        let new_bot = Bot {
                            numbers: vec![numbers[0]],
                        };
                        new_bots.insert(low_bot, new_bot);
                    }
                },
                Destination::Output(out) => {
                    outputs.insert(out, numbers[0]);
                }
            }
            match high {
                Destination::Bot(high_bot) => match new_bots.get_mut(&high_bot) {
                    Some(new_bot) => {
                        new_bot.give(numbers[1]);
                    }
                    None => {
                        let new_bot = Bot {
                            numbers: vec![numbers[1]],
                        };
                        new_bots.insert(high_bot, new_bot);
                    }
                },
                Destination::Output(out) => {
                    outputs.insert(out, numbers[1]);
                }
            }
        } else {
            match new_bots.get_mut(&i) {
                Some(new_bot) => {
                    if !bot.numbers().is_empty() {
                        new_bot.give(bot.numbers()[0]);
                    }
                }
                None => {
                    new_bots.insert(
                        *i,
                        Bot {
                            numbers: bot.numbers().to_vec(),
                        },
                    );
                }
            }
        }
    }
    State {
        bots: new_bots,
        outputs,
    }
}

pub fn find_bot(state: &State, low: u32, high: u32) -> Option<u32> {
    for (k, bot) in &state.bots {
        if bot.numbers() == [low, high] {
            return Some(*k);
        }
    }
    None
}

pub fn first_bot_with_numbers(init: &[Initial], transfers: &Transfers, low: u32, high: u32) -> u32 {
    let mut state = set_up_bots(init);
    loop {
        if let Some(bot) = find_bot(&state, low, high) {
            return bot;
        }
        state = next_state(&state, transfers);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_instructions_test_1() {
        use Destination::*;
        let lines = [
            "bot 18 gives low to bot 10 and high to bot 13",
            "value 7 goes to bot 19",
            "bot 7 gives low to bot 34 and high to output 16",
            "bot 9 gives low to output 50 and high to bot 10",
            "value 10 goes to bot 5",
        ];
        let transfers = Transfers::from_iter([
            (18, (Bot(10), Bot(13))),
            (7, (Bot(34), Output(16))),
            (9, (Output(50), Bot(10))),
        ]);
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

    #[test]
    fn set_up_bots_test_1() {
        let mut bots = Bots::new();
        bots.insert(
            15,
            Bot {
                numbers: vec![1, 4],
            },
        );
        bots.insert(3, Bot { numbers: vec![10] });
        bots.insert(8, Bot { numbers: vec![11] });
        let initials = [(4, 15), (10, 3), (11, 8), (1, 15)];
        let correct = State {
            bots,
            ..State::default()
        };
        assert_eq!(set_up_bots(&initials), correct);
    }

    #[test]
    fn next_state_test_1() {
        let initials = [(4, 15), (10, 3), (11, 8), (1, 15)];
        let mut transfers = Transfers::new();
        transfers.insert(15, (Destination::Bot(3), Destination::Bot(8)));
        transfers.insert(3, (Destination::Bot(8), Destination::Output(3)));
        transfers.insert(8, (Destination::Output(10), Destination::Bot(12)));
        let next_1 = State {
            bots: Bots::from_iter([
                (15, Bot { numbers: vec![] }),
                (
                    8,
                    Bot {
                        numbers: vec![4, 11],
                    },
                ),
                (
                    3,
                    Bot {
                        numbers: vec![1, 10],
                    },
                ),
            ]),
            outputs: HashMap::new(),
        };
        let initial = set_up_bots(&initials);
        assert_eq!(next_state(&initial, &transfers), next_1);
        let next_2 = State {
            bots: Bots::from_iter([
                (15, Bot { numbers: vec![] }),
                (8, Bot { numbers: vec![1] }),
                (3, Bot { numbers: vec![] }),
                (12, Bot { numbers: vec![11] }),
            ]),
            outputs: HashMap::<u32, u32>::from_iter([(3, 10), (10, 4)]),
        };
        assert_eq!(next_state(&next_1, &transfers), next_2);
    }

    #[test]
    fn find_bot_test_1() {
        let initials = [(4, 15), (10, 3), (11, 8), (1, 15)];
        let initial = set_up_bots(&initials);
        assert_eq!(find_bot(&initial, 1, 4), Some(15));
    }

    #[test]
    fn find_bot_test_2() {
        let initials = [(4, 15), (10, 3), (11, 8), (1, 15)];
        let initial = set_up_bots(&initials);
        assert_eq!(find_bot(&initial, 8, 4), None);
    }

    #[test]
    fn first_bot_with_numbers_test_1() {
        let initials = [(4, 15), (10, 3), (11, 8), (1, 15)];
        let mut transfers = Transfers::new();
        transfers.insert(15, (Destination::Bot(3), Destination::Bot(8)));
        transfers.insert(3, (Destination::Bot(8), Destination::Output(3)));
        transfers.insert(8, (Destination::Output(10), Destination::Bot(12)));
        assert_eq!(first_bot_with_numbers(&initials, &transfers, 1, 10), 3);
    }
}
