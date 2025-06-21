use sscanf::sscanf;
use std::{collections::HashSet, str::FromStr};

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

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Node {
    name: String,
    weight: u64,
    total_weight: u64,
    children: Vec<Node>,
}

#[derive(Debug, PartialEq)]
pub enum NodeError {
    NoRootNode,
    NodeMissing(String),
}

impl Node {
    fn create_node(parsed: &[ParsedNode], name: &str) -> Result<Self, NodeError> {
        let current = parsed
            .iter()
            .find(|n| n.name == name)
            .ok_or(NodeError::NodeMissing(name.to_owned()))?;
        let mut children = vec![];
        let mut total_weight = current.weight;
        for child in &current.children {
            let child_node = Node::create_node(parsed, child)?;
            total_weight += child_node.total_weight;
            children.push(child_node);
        }
        Ok(Node {
            name: current.name.to_owned(),
            weight: current.weight,
            total_weight,
            children,
        })
    }

    pub fn new(parsed: &[ParsedNode]) -> Result<Self, NodeError> {
        let parent = bottom_node(parsed).ok_or(NodeError::NoRootNode)?;
        Node::create_node(parsed, &parent.name)
    }
}

pub fn unbalanced_child(parent: &Node) -> Option<&Node> {
    let total_weight: u64 = parent.children.iter().map(|child| child.total_weight).sum();
    let child_count = parent.children.len();
    for node in &parent.children {
        if node.total_weight * child_count as u64 != total_weight {
            return unbalanced_child(node).or(Some(node));
        }
    }
    None
}

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

    #[test]
    fn node_new_test_1() {
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
        let correct = Node {
            name: "ghi".to_owned(),
            weight: 4,
            total_weight: 107,
            children: vec![
                Node {
                    name: "def".to_owned(),
                    weight: 5,
                    total_weight: 91,
                    children: vec![
                        Node {
                            name: "abc".to_owned(),
                            weight: 1,
                            total_weight: 1,
                            children: vec![],
                        },
                        Node {
                            name: "mno".to_owned(),
                            weight: 85,
                            total_weight: 85,
                            children: vec![],
                        },
                    ],
                },
                Node {
                    name: "jkl".to_owned(),
                    weight: 2,
                    total_weight: 12,
                    children: vec![Node {
                        name: "pqr".to_owned(),
                        weight: 10,
                        total_weight: 10,
                        children: vec![],
                    }],
                },
            ],
        };
        assert_eq!(Node::new(&nodes), Ok(correct));
    }

    #[test]
    fn node_new_test_2() {
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
        assert_eq!(Node::new(&nodes), Err(NodeError::NoRootNode))
    }

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
        let nodes_vec: Vec<_> = nodes
            .iter()
            .map(|n| n.parse::<ParsedNode>().unwrap())
            .collect();
        let node_tree = Node::new(&nodes_vec).unwrap();
        assert_eq!(unbalanced_child(&node_tree), None);
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
        let nodes_vec: Vec<_> = nodes
            .iter()
            .map(|n| n.parse::<ParsedNode>().unwrap())
            .collect();
        let node_tree = Node::new(&nodes_vec).unwrap();
        assert_eq!(
            unbalanced_child(&node_tree),
            Some(&Node {
                name: "mno".to_owned(),
                weight: 1,
                total_weight: 1,
                children: vec![],
            })
        );
    }
}
