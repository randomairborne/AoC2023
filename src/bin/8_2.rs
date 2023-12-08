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
    let mut cycle_times = Vec::new();
    for starting_code in starting_codes {
        let mut steps = 0;
        let mut last = codes_map.get(&starting_code).unwrap();
        for direction in pattern.iter().cycle() {
            steps += 1;
            let next = match direction {
                Direction::Left => &last.left,
                Direction::Right => &last.right,
            };
            if next[2] == 'Z' {
                break;
            }
            last = codes_map.get(next).unwrap();
        }
        cycle_times.push(steps);
    }
    eprintln!("{cycle_times:?}");
    let steps = recursive_lcm(&cycle_times);
    println!("{steps}");
}

fn recursive_lcm(data: &[i64]) -> i64 {
    if data.len() < 2 {
        panic!("recursive_lcm must have at least 2 data points");
    } else if data.len() == 2 {
        lcm(data[0], data[1])
    } else {
        lcm(data[0], recursive_lcm(&data[1..]))
    }
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

// Stolen from num-bigint- reimplementing gcd and lcm was too hard

#[inline]
fn gcd(a: i64, b: i64) -> i64 {
    // Use Stein's algorithm
    let mut m = a;
    let mut n = b;
    if m == 0 || n == 0 {
        return m | n;
    }

    // find common factors of 2
    let shift = (m | n).trailing_zeros();

    // divide n and m by 2 until odd
    m >>= m.trailing_zeros();
    n >>= n.trailing_zeros();

    while m != n {
        if m > n {
            m -= n;
            m >>= m.trailing_zeros();
        } else {
            n -= m;
            n >>= n.trailing_zeros();
        }
    }
    m << shift
}

fn lcm(a: i64, b: i64) -> i64 {
    gcd_lcm(a, b).1
}

fn gcd_lcm(a: i64, b: i64) -> (i64, i64) {
    if a == 0 && b == 0 {
        return (0, 0);
    }
    let gcd = gcd(a, b);
    let lcm = a * (b / gcd);
    (gcd, lcm)
}
