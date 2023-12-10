fn main() {
    let input = std::fs::read_to_string("./input.txt").unwrap();
    let mut sum = 0;
    for line in input.lines().map(|v| {
        v.split(' ')
            .map(|v| v.parse::<i64>().unwrap())
            .collect::<Vec<i64>>()
    }) {
        sum += process_line(line);
        println!();
    }
    println!("{sum}");
}

fn process_line(line: Vec<i64>) -> i64 {
    let mut total = 0;
    let tree = generate_lines(vec![line]);
    let mut idx = 1;
    while idx <= tree.len() - 1 {
        let line = &tree[idx];
        println!("{line:?}");
        let below = &tree[idx - 1];
        total += extend_line(line, below);
        idx += 1;
    }
    total
}

fn generate_lines(mut lines: Vec<Vec<i64>>) -> Vec<Vec<i64>> {
    let top = &lines[lines.len() - 1];
    let mut next = Vec::new();
    let mut all_zeros = true;
    for (idx, item) in top.iter().enumerate() {
        if idx == 0 {
            continue;
        }
        if *item != 0 {
            all_zeros = false;
        }
        next.push(item - top[idx - 1]);
    }
    if all_zeros {
        return lines;
    }
    lines.push(next);
    generate_lines(lines)
}

fn extend_line(existing: &[i64], below: &[i64]) -> i64 {
    if existing.iter().all(|v| *v == 0) {
        return 0;
    }
    let existing_last = existing.len() - 1;
    let below_last = existing.len() - 2;
    let last_existing = existing[existing_last];
    let last_below = below[below_last];
    last_existing + last_below
}
