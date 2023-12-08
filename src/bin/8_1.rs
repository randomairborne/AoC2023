use std::collections::HashMap;

type Code = [char; 3];

const START: Code = ['A'; 3];
const END: Code = ['Z'; 3];

fn main() {
    let input = std::fs::read_to_string("./input.txt").unwrap();
    let mut lines_iter = input.lines().into_iter();
    let pattern: Vec<Direction> = lines_iter
        .next()
        .unwrap()
        .chars()
        .map(Direction::from_char)
        .collect();
    let mut codes_map: HashMap<Code, CodePair> = HashMap::new();
    for line in lines_iter {
        if line.is_empty() {
            continue;
        }
        let mut pairs = line.splitn(2, '=').map(|v| v.trim());
        let code: Code = pairs
            .next()
            .unwrap()
            .chars()
            .collect::<Vec<char>>()
            .try_into()
            .unwrap();
        let pair: Vec<Code> = pairs
            .next()
            .unwrap()
            .splitn(2, ',')
            .map(|v| {
                v.trim_matches(|v: char| !v.is_ascii_alphanumeric())
                    .chars()
                    .collect::<Vec<char>>()
                    .try_into()
                    .unwrap()
            })
            .collect();
        let left = pair[0];
        let right = pair[1];
        codes_map.insert(code, CodePair { left, right });
    }
    let mut last = codes_map.get(&START).unwrap();
    let mut steps = 1;
    for item in pattern.iter().cycle() {
        steps += 1;
        let next = match item {
            Direction::Left => &last.left,
            Direction::Right => &last.right,
        };
        if *next == END {
            break;
        }
        last = codes_map.get(next).unwrap();
    }
    println!("{steps}");
}

#[derive(Copy, Clone, Hash, Eq, PartialEq)]
struct CodePair {
    left: Code,
    right: Code,
}

enum Direction {
    Left,
    Right,
}

impl Direction {
    fn from_char(c: char) -> Self {
        match c.to_ascii_uppercase() {
            'L' => Self::Left,
            'R' => Self::Right,
            _ => panic!("Got {c} neither L nor R"),
        }
    }
}
