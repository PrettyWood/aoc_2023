use lazy_static::lazy_static;
use num::integer::lcm;
use regex::Regex;
use std::collections::{BTreeMap, BTreeSet};

fn main() {
    println!("Part 1: {}", part1(include_str!("input.txt")));
    println!("Part 2: {}", part2(include_str!("input.txt")));
}

#[derive(Debug)]
struct Map {
    instructions: Vec<Direction>,
    nodes: BTreeMap<String, (String, String)>,
}

#[derive(Debug)]
enum Direction {
    Left,
    Right,
}

impl From<char> for Direction {
    fn from(c: char) -> Self {
        match c {
            'L' => Self::Left,
            'R' => Self::Right,
            _ => panic!("Only L or R allowed"),
        }
    }
}

lazy_static! {
    static ref RE: Regex = Regex::new(r"(\w{3}) = \((\w{3}), (\w{3})\)").unwrap();
}

// LRL = (MCG, TRC)
fn parse_node_line(input: &str) -> (String, (String, String)) {
    let caps = RE.captures(input).unwrap();
    (
        caps[1].to_string(),
        (caps[2].to_string(), caps[3].to_string()),
    )
}

impl From<&str> for Map {
    fn from(input: &str) -> Self {
        Self {
            instructions: input
                .lines()
                .next()
                .unwrap()
                .chars()
                .map(Direction::from)
                .collect(),
            nodes: input.lines().skip(2).map(parse_node_line).collect(),
        }
    }
}

impl Map {
    fn get_count_for_node(&self, start_pos: &str, is_end_pos: fn(&str) -> bool) -> usize {
        let mut current_pos = start_pos;
        for (count, direction) in self.instructions.iter().cycle().enumerate() {
            if is_end_pos(current_pos) {
                return count;
            }

            let (left, right) = self.nodes.get(current_pos).unwrap();
            current_pos = match direction {
                Direction::Left => left,
                Direction::Right => right,
            };
        }
        panic!("No end position found");
    }
}

pub fn part1(input: &str) -> usize {
    let map = Map::from(input);
    let start_pos = "AAA";
    let is_end_pos = |s: &str| s == "ZZZ";
    map.get_count_for_node(start_pos, is_end_pos)
}

pub fn part2(input: &str) -> usize {
    let map = Map::from(input);
    let all_start_pos = map
        .nodes
        .keys()
        .filter(|k| k.ends_with('A'))
        .collect::<BTreeSet<_>>();
    let counts = all_start_pos
        .iter()
        .map(|k| map.get_count_for_node(k, |s| s.ends_with('Z')))
        .collect::<BTreeSet<_>>();
    counts.iter().fold(1, |acc, x| lcm(acc, *x))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        assert_eq!(
            parse_node_line("LRL = (MCG, TRC)"),
            ("LRL".to_string(), ("MCG".to_string(), "TRC".to_string()))
        );
    }

    #[test]
    fn test_part1() {
        let input = r#"RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)"#;
        assert_eq!(part1(input), 2);
    }

    #[test]
    fn test_part1_repeat() {
        let input = r#"LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)"#;
        assert_eq!(part1(input), 6);
    }

    #[test]
    fn test_part2() {
        let input = r#"LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)"#;
        assert_eq!(part2(input), 6);
    }
}
