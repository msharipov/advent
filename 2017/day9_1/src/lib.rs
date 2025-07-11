use std::str::FromStr;

use thiserror::Error;

#[derive(Debug, PartialEq)]
pub enum GroupPart {
    Group(Vec<GroupPart>),
    Garbage,
}

#[derive(Debug, Error, PartialEq)]
#[error("failed to parse group")]
pub struct GroupParseError();

fn parse_garbage(s: &str) -> Result<usize, GroupParseError> {
    if s.is_empty() {
        return Err(GroupParseError());
    }
    let chars: Vec<_> = s.chars().collect();
    let len = chars.len();
    if chars[0] != '<' || len < 2 {
        return Err(GroupParseError());
    }
    let mut current_index: usize = 0;
    let mut escaped = false;
    loop {
        current_index += 1;
        if current_index >= len {
            return Err(GroupParseError());
        }
        if escaped {
            escaped = false;
            continue;
        }
        match chars[current_index] {
            '!' => {
                escaped = true;
            }
            '>' if !escaped => {
                break;
            }
            _ => {}
        }
    }
    Ok(current_index + 1)
}

fn parse_group(s: &str) -> Result<(GroupPart, usize), GroupParseError> {
    if s.is_empty() {
        return Err(GroupParseError());
    }
    let chars: Vec<_> = s.chars().collect();
    let len = chars.len();
    if chars[0] != '{' || len < 2 {
        return Err(GroupParseError());
    }
    let mut current_index: usize = 0;
    let mut contents = vec![];
    loop {
        current_index += 1;
        if current_index >= len {
            return Err(GroupParseError());
        }
        match chars[current_index] {
            '}' => {
                return Ok((GroupPart::Group(contents), current_index + 1));
            }
            _ => return Err(GroupParseError()),
        }
    }
}

impl FromStr for GroupPart {
    type Err = GroupParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() {
            return Err(GroupParseError());
        }
        let chars: Vec<_> = s.chars().collect();
        match chars[0] {
            '<' => {
                let garbage_len = parse_garbage(s)?;
                if garbage_len != chars.len() {
                    Err(GroupParseError())
                } else {
                    Ok(GroupPart::Garbage)
                }
            }
            '{' => todo!(),
            _ => Err(GroupParseError()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_group_test_1() {
        assert_eq!(parse_group(""), Err(GroupParseError()));
    }

    #[test]
    fn parse_group_test_2() {
        assert_eq!(parse_group("<asdfasdf>"), Err(GroupParseError()));
    }

    #[test]
    fn parse_group_test_3() {
        assert_eq!(parse_group("{asdfasdfad"), Err(GroupParseError()));
    }

    #[test]
    fn parse_group_test_4() {
        assert_eq!(parse_group("{}"), Ok((GroupPart::Group(vec![]), 2)));
    }

    #[test]
    fn parse_garbage_test_1() {
        assert_eq!(parse_garbage(""), Err(GroupParseError()));
    }

    #[test]
    fn parse_garbage_test_2() {
        assert_eq!(parse_garbage("<sdfad"), Err(GroupParseError()));
    }

    #[test]
    fn parse_garbage_test_3() {
        assert_eq!(parse_garbage("a123sad<sadf>"), Err(GroupParseError()));
    }

    #[test]
    fn parse_garbage_test_4() {
        assert_eq!(parse_garbage("<sdf!>12mfp"), Err(GroupParseError()));
    }

    #[test]
    fn parse_garbage_test_5() {
        assert_eq!(parse_garbage("<>"), Ok(2));
    }

    #[test]
    fn parse_garbage_test_6() {
        assert_eq!(parse_garbage("<asdfasdf>"), Ok(10));
    }

    #[test]
    fn parse_garbage_test_7() {
        assert_eq!(parse_garbage("<sadfa!>sdf>af23"), Ok(12));
    }

    #[test]
    fn grouppart_parse_test_1() {
        assert_eq!("<>".parse(), Ok(GroupPart::Garbage));
    }

    #[test]
    fn grouppart_parse_test_2() {
        assert_eq!("".parse::<GroupPart>(), Err(GroupParseError()));
    }

    #[test]
    fn grouppart_parse_test_3() {
        assert_eq!("<aavbcvb1234!@#sdf>".parse(), Ok(GroupPart::Garbage));
    }

    #[test]
    fn grouppart_parse_test_4() {
        assert_eq!("<sasdfasdf!>".parse::<GroupPart>(), Err(GroupParseError()));
    }

    #[test]
    fn grouppart_parse_test_5() {
        assert_eq!("<adfad<fgad<f!>asdfasdf>".parse(), Ok(GroupPart::Garbage));
    }

    #[test]
    fn grouppart_parse_test_6() {
        assert_eq!(
            "<23405<asdfa!>dasf".parse::<GroupPart>(),
            Err(GroupParseError())
        );
    }

    #[test]
    fn grouppart_parse_test_7() {
        assert_eq!(
            "<23405k<asdf>dsaf>".parse::<GroupPart>(),
            Err(GroupParseError())
        );
    }

    #[test]
    fn grouppart_parse_test_8() {
        assert_eq!(
            "asdfas<drf!><123>".parse::<GroupPart>(),
            Err(GroupParseError())
        );
    }
}
