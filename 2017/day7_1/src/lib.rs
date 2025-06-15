use sscanf::sscanf;
use std::{collections::HashSet, str::FromStr};

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Node {
    name: String,
    weight: u64,
    children: Vec<String>,
}

impl Node {
    pub fn new(name: &str, weight: u64, children: &[&str]) -> Self {
        Node {
            name: name.to_owned(),
            weight,
            children: children.iter().map(|&s| s.to_owned()).collect(),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct ParseNodeError {
    bad_string: String,
}

impl FromStr for Node {
    type Err = ParseNodeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Ok((name, weight)) = sscanf!(s, "{&str:/[a-z]+/} ({u64})") {
            return Ok(Node::new(name, weight, &[]));
        }
        if let Ok((name, weight, children)) =
            sscanf!(s, "{&str:/[a-z]+/} ({u64}) -> {&str:/[a-z, ]+/}")
        {
            let children: Vec<_> = children.split(',').map(|child| child.trim()).collect();
            if children.is_empty() {
                return Err(ParseNodeError {
                    bad_string: s.to_owned(),
                });
            }
            return Ok(Node::new(name, weight, &children));
        }
        Err(ParseNodeError {
            bad_string: s.to_owned(),
        })
    }
}

pub fn bottom_node(nodes: &[Node]) -> Option<&Node> {
    let mut all_children = HashSet::new();
    for node in nodes {
        for child in &node.children {
            all_children.insert(child.to_owned());
        }
    }
    nodes.iter().find(|n| !all_children.contains(&n.name))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn node_from_str_test_1() {
        let correct = Node::new("qwerty", 125, &[]);
        assert_eq!(Ok(correct), "qwerty (125)".parse())
    }

    #[test]
    fn node_from_str_test_2() {
        let correct = Node::new("qwerty", 125, &["abcd"]);
        assert_eq!(Ok(correct), "qwerty (125) -> abcd".parse());
    }

    #[test]
    fn node_from_str_test_3() {
        let correct = Node::new("qwerty", 125, &["abcd", "efg", "hijklm"]);
        assert_eq!(Ok(correct), "qwerty (125) -> abcd, efg, hijklm".parse());
    }

    #[test]
    fn node_from_str_test_4() {
        let s = "abcd ()";
        assert_eq!(
            Err(ParseNodeError {
                bad_string: s.to_owned()
            }),
            s.parse::<Node>()
        );
    }

    #[test]
    fn node_from_str_test_5() {
        let s = "abcd (456) ->";
        assert_eq!(
            Err(ParseNodeError {
                bad_string: s.to_owned()
            }),
            s.parse::<Node>()
        );
    }

    #[test]
    fn node_from_str_test_6() {
        let s = "abcd (123) -> 123, 456";
        assert_eq!(
            Err(ParseNodeError {
                bad_string: s.to_owned()
            }),
            s.parse::<Node>()
        );
    }

    #[test]
    fn bottom_node_test_1() {
        let nodes = [
            "abc (1)",
            "def (5) -> abc, mno",
            "ghi (4) -> def, jkl",
            "jkl (2) -> pqr",
            "mno (85)",
            "pqr (10)",
        ];
        let nodes: Vec<_> = nodes.iter().map(|n| n.parse::<Node>().unwrap()).collect();
        assert_eq!(
            bottom_node(&nodes),
            Some(&Node::new("ghi", 4, &["def", "jkl"]))
        );
    }

    #[test]
    fn bottom_node_test_2() {
        let nodes = [
            "abc (1)",
            "def (5) -> abc, mno",
            "ghi (4) -> def, jkl",
            "jkl (2) -> pqr",
            "mno (85)",
            "pqr (10) -> ghi",
        ];
        let nodes: Vec<_> = nodes.iter().map(|n| n.parse::<Node>().unwrap()).collect();
        assert_eq!(bottom_node(&nodes), None);
    }
}
