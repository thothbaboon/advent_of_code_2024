use lazy_static::lazy_static;
use std::collections::{BinaryHeap, HashMap, HashSet};

use crate::read_input;

#[derive(Copy, Clone)]
pub enum Direction {
    UP,
    DOWN,
    RIGHT,
    LEFT,
}

impl Direction {
    pub fn as_str(&self) -> &'static str {
        match self {
            Direction::UP => "^",
            Direction::DOWN => "v",
            Direction::RIGHT => ">",
            Direction::LEFT => "<",
        }
    }
}

lazy_static! {
    static ref NUMERIC_KEYPAD: HashMap<&'static str, Vec<(Direction, &'static str)>> = {
        let mut m = HashMap::new();

        // Top row
        m.insert("7", vec![(Direction::RIGHT, "8"), (Direction::DOWN, "4")]);
        m.insert("8", vec![(Direction::LEFT, "7"), (Direction::RIGHT, "9"), (Direction::DOWN, "5")]);
        m.insert("9", vec![(Direction::LEFT, "8"), (Direction::DOWN, "6")]);

        // Middle row
        m.insert("4", vec![(Direction::UP, "7"), (Direction::RIGHT, "5"), (Direction::DOWN, "1")]);
        m.insert("5", vec![(Direction::UP, "8"), (Direction::LEFT, "4"), (Direction::RIGHT, "6"), (Direction::DOWN, "2")]);
        m.insert("6", vec![(Direction::UP, "9"), (Direction::LEFT, "5"), (Direction::DOWN, "3")]);

        // Bottom row
        m.insert("1", vec![(Direction::UP, "4"), (Direction::RIGHT, "2")]);
        m.insert("2", vec![(Direction::UP, "5"), (Direction::LEFT, "1"), (Direction::RIGHT, "3"), (Direction::DOWN, "0")]);
        m.insert("3", vec![(Direction::UP, "6"), (Direction::LEFT, "2"), (Direction::DOWN, "A")]);

        // Bottom-most row
        m.insert("0", vec![(Direction::UP, "2"), (Direction::RIGHT, "A")]);
        m.insert("A", vec![(Direction::UP, "3"), (Direction::LEFT, "0")]);

        m
    };

    static ref DIRECTIONAL_KEYPAD: HashMap<&'static str, Vec<(Direction, &'static str)>> = {
        let mut m = HashMap::new();

        // Top row
        m.insert("^", vec![(Direction::RIGHT, "A"), (Direction::DOWN, "v")]);
        m.insert("A", vec![(Direction::LEFT, "^"), (Direction::DOWN, ">")]);

        // Bottom row
        m.insert("<", vec![(Direction::RIGHT, "v")]);
        m.insert("v", vec![(Direction::UP, "^"), (Direction::LEFT, "<"), (Direction::RIGHT, ">")]);
        m.insert(">", vec![(Direction::UP, "A"), (Direction::LEFT, "v")]);

        m
    };
}

#[derive(Eq, PartialEq)]
struct DijkstraCandidate {
    key: String,
    sequence: String,
}

impl Ord for DijkstraCandidate {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other
            .sequence
            .len()
            .cmp(&self.sequence.len())
            .then_with(|| self.key.cmp(&other.key))
    }
}
impl PartialOrd for DijkstraCandidate {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn numeric_keypad_dijkstra(from: &str, to: &str) -> Vec<String> {
    let start = DijkstraCandidate {
        key: from.to_string(),
        sequence: "".to_string(),
    };

    let mut queue = BinaryHeap::new();
    queue.push(start);

    let mut shortest_sequences: HashMap<String, Vec<String>> = HashMap::new();

    while let Some(candidate) = queue.pop() {
        let sequences = shortest_sequences.entry(candidate.key.clone()).or_default();

        if sequences.len() > 0 && sequences[0].len() != candidate.sequence.len() {
            continue;
        }

        sequences.push(candidate.sequence.clone());

        let directions = NUMERIC_KEYPAD.get(&candidate.key.as_str()).unwrap();

        for (sequence, target_key) in directions.iter() {
            let mut s = candidate.sequence.clone();
            s.push_str(&sequence.as_str());
            queue.push(DijkstraCandidate {
                key: target_key.to_string(),
                sequence: s,
            });
        }
    }

    if let Some(v) = shortest_sequences.get(to) {
        v.clone()
    } else {
        vec![]
    }
}

fn directional_keypad_dijkstra(from: &str, to: &str) -> Vec<String> {
    let start = DijkstraCandidate {
        key: from.to_string(),
        sequence: "".to_string(),
    };

    let mut queue = BinaryHeap::new();
    queue.push(start);

    let mut shortest_sequences: HashMap<String, Vec<String>> = HashMap::new();

    while let Some(candidate) = queue.pop() {
        let sequences = shortest_sequences.entry(candidate.key.clone()).or_default();

        if sequences.len() > 0 && sequences[0].len() != candidate.sequence.len() {
            continue;
        }

        sequences.push(candidate.sequence.clone());

        let directions = DIRECTIONAL_KEYPAD.get(&candidate.key.as_str()).unwrap();

        for (sequence, target_key) in directions.iter() {
            let mut s = candidate.sequence.clone();
            s.push_str(&sequence.as_str());
            queue.push(DijkstraCandidate {
                key: target_key.to_string(),
                sequence: s,
            });
        }
    }

    if let Some(v) = shortest_sequences.get(to) {
        v.clone()
    } else {
        vec![]
    }
}

fn find_numeric_keypad_sequence(code: &str) -> HashSet<String> {
    let mut start_key = "A".to_string();
    let mut sequences_parts = vec![];

    for c in code.chars() {
        sequences_parts.push(numeric_keypad_dijkstra(&start_key, &c.to_string()));
        sequences_parts.push(vec!["A".to_string()]);
        start_key = c.to_string();
    }

    let mut result = vec!["".to_string()];

    for part in sequences_parts {
        let mut new_result = vec![];
        for existing in result {
            for sequence in &part {
                let mut combined = existing.clone();
                combined.push_str(sequence);
                new_result.push(combined);
            }
        }
        result = new_result;
    }

    if let Some(min) = result.iter().min_by_key(|v| v.len()) {
        result
            .iter()
            .filter(|v| v.len() == min.len())
            .cloned()
            .collect()
    } else {
        HashSet::new()
    }
}

fn find_directional_keypad_sequence(sq: &str) -> Vec<String> {
    let mut start_key = "A".to_string();
    let mut sequences_parts = vec![];

    for c in sq.chars() {
        sequences_parts.push(directional_keypad_dijkstra(&start_key, &c.to_string()));
        sequences_parts.push(vec!["A".to_string()]);
        start_key = c.to_string();
    }

    let mut result = vec!["".to_string()];

    for part in sequences_parts {
        let mut new_result = vec![];
        for existing in result {
            for sequence in &part {
                let mut combined = existing.clone();
                combined.push_str(sequence);
                new_result.push(combined);
            }
        }
        result = new_result;
    }

   result
}

pub fn run_part_1() {
    let s = read_input("day21", "input.txt")
        .unwrap()
        .filter_map(Result::ok)
        .map(|code| {
            let r1_codes = find_numeric_keypad_sequence(&code);

            let r2_codes = r1_codes.iter().map(|v| find_directional_keypad_sequence(v)).flatten().collect::<Vec<String>>();
            let r2_codes = if let Some(min) = r2_codes.iter().min_by_key(|v| v.len()) {
                r2_codes
                    .iter()
                    .filter(|v| v.len() == min.len())
                    .cloned()
                    .collect()
            } else {
                HashSet::new()
            };

            let r3_codes = r2_codes.iter().map(|v| find_directional_keypad_sequence(v)).flatten().collect::<Vec<String>>();
            let r3_codes = if let Some(min) = r3_codes.iter().min_by_key(|v| v.len()) {
                r3_codes
                    .iter()
                    .filter(|v| v.len() == min.len())
                    .cloned()
                    .collect()
            } else {
                HashSet::new()
            };

            let s_len = r3_codes.iter().next().map_or(0, |s| s.len());
            let digit = code[0..3].parse::<usize>().unwrap_or(0);

            s_len * digit
        })
        .sum::<usize>();
    println!("{:?}", s);
}
