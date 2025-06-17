use sscanf::sscanf;
use std::{
    collections::{HashMap, HashSet},
    str::FromStr,
};

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct ParsedNode {
    name: String,
    weight: u64,
    children: Vec<String>,
}

impl ParsedNode {
    pub fn new(name: &str, weight: u64, children: &[&str]) -> Self {
        ParsedNode {
            name: name.to_owned(),
            weight,
            children: children.iter().map(|&s| s.to_owned()).collect(),
        }
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }
}

#[derive(Debug, PartialEq)]
pub struct ParseNodeError {
    bad_string: String,
}

impl FromStr for ParsedNode {
    type Err = ParseNodeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Ok((name, weight)) = sscanf!(s, "{&str:/[a-z]+/} ({u64})") {
            return Ok(ParsedNode::new(name, weight, &[]));
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
            return Ok(ParsedNode::new(name, weight, &children));
        }
        Err(ParseNodeError {
            bad_string: s.to_owned(),
        })
    }
}

pub fn bottom_node(nodes: &[ParsedNode]) -> Option<&ParsedNode> {
    let mut all_children = HashSet::new();
    for node in nodes {
        for child in &node.children {
            all_children.insert(child.to_owned());
        }
    }
    nodes.iter().find(|n| !all_children.contains(&n.name))
}

/*
pub fn unbalanced_child(nodes: &HashMap<String, Node>, parent: &str) -> Option<String> {
    let parent = nodes.get(parent)?;
    let mut children = vec![];
    for node in &parent.children {
        children.push(nodes.get(node)?);
    }
    let total_weight: u64 = children.iter().map(|child| child.weight).sum();
    for node in children {
        if node.weight * 3 != total_weight {
            return unbalanced_child(nodes, &node.name).or(Some(node.name.to_owned()));
        }
    }
    None
}
*/

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn node_from_str_test_1() {
        let correct = ParsedNode::new("qwerty", 125, &[]);
        assert_eq!(Ok(correct), "qwerty (125)".parse())
    }

    #[test]
    fn node_from_str_test_2() {
        let correct = ParsedNode::new("qwerty", 125, &["abcd"]);
        assert_eq!(Ok(correct), "qwerty (125) -> abcd".parse());
    }

    #[test]
    fn node_from_str_test_3() {
        let correct = ParsedNode::new("qwerty", 125, &["abcd", "efg", "hijklm"]);
        assert_eq!(Ok(correct), "qwerty (125) -> abcd, efg, hijklm".parse());
    }

    #[test]
    fn node_from_str_test_4() {
        let s = "abcd ()";
        assert_eq!(
            Err(ParseNodeError {
                bad_string: s.to_owned()
            }),
            s.parse::<ParsedNode>()
        );
    }

    #[test]
    fn node_from_str_test_5() {
        let s = "abcd (456) ->";
        assert_eq!(
            Err(ParseNodeError {
                bad_string: s.to_owned()
            }),
            s.parse::<ParsedNode>()
        );
    }

    #[test]
    fn node_from_str_test_6() {
        let s = "abcd (123) -> 123, 456";
        assert_eq!(
            Err(ParseNodeError {
                bad_string: s.to_owned()
            }),
            s.parse::<ParsedNode>()
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
        let nodes: Vec<_> = nodes
            .iter()
            .map(|n| n.parse::<ParsedNode>().unwrap())
            .collect();
        assert_eq!(
            bottom_node(&nodes),
            Some(&ParsedNode::new("ghi", 4, &["def", "jkl"]))
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
        let nodes: Vec<_> = nodes
            .iter()
            .map(|n| n.parse::<ParsedNode>().unwrap())
            .collect();
        assert_eq!(bottom_node(&nodes), None);
    }

    /*
    #[test]
    fn unbalanced_child_test_1() {
        let nodes = [
            "abc (34) -> def, ghi, jkl",
            "def (8) -> mno, pqr",
            "mno (2)",
            "pqr (2)",
            "ghi (12)",
            "jkl (12)",
        ];
        let nodes: HashMap<String, Node> = HashMap::from_iter(
            nodes
                .iter()
                .map(|n| n.parse::<Node>().unwrap())
                .map(|n| (n.name.to_owned(), n)),
        );
        assert_eq!(unbalanced_child(&nodes, "abc"), None);
    }

    #[test]
    fn unbalanced_child_test_2() {
        let nodes = [
            "abc (34) -> def, ghi, jkl",
            "def (8) -> mno, pqr",
            "mno (1)",
            "pqr (2)",
            "ghi (12)",
            "jkl (12)",
        ];
        let nodes: HashMap<String, Node> = HashMap::from_iter(
            nodes
                .iter()
                .map(|n| n.parse::<Node>().unwrap())
                .map(|n| (n.name.to_owned(), n)),
        );
        assert_eq!(unbalanced_child(&nodes, "abc"), None);
    }
    */
}
