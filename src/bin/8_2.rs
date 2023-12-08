use std::collections::HashMap;

type Code = [char; 3];

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
    let mut starting_codes: Vec<Code> = Vec::new();
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
        if code[2] == 'A' {
            starting_codes.push(code);
        }
        codes_map.insert(code, CodePair { left, right });
    }
    let mut states = starting_codes;
    let mut steps = 0;
    for direction in pattern.iter().cycle() {
        steps += 1;
        let mut all_z = true;
        for state in states.iter_mut() {
            let next_pair = codes_map.get(state).unwrap();
            let next = match direction {
                Direction::Left => &next_pair.left,
                Direction::Right => &next_pair.right,
            };
            if next[2] != 'Z' {
                all_z = false;
            }
            *state = *next;
        }
        if all_z {
            break;
        }
    }
    println!("{steps}");
}

#[derive(Copy, Clone, Hash, Eq, PartialEq, Debug)]
struct CodePair {
    left: Code,
    right: Code,
}

#[derive(Copy, Clone)]
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
