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

fn parse_garbage(s: &str) -> Result<GroupPart, GroupParseError> {
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
    if current_index != len - 1 {
        return Err(GroupParseError());
    }
    Ok(GroupPart::Garbage)
}

impl FromStr for GroupPart {
    type Err = GroupParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() {
            return Err(GroupParseError());
        }
        let chars: Vec<_> = s.chars().collect();
        if chars[0] == '<' {
            parse_garbage(s)
        } else {
            Err(GroupParseError())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
