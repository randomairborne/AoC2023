fn main() {
    let input = std::fs::read_to_string("./input.txt").unwrap();
    let mut sum = 0;
    for line in input.lines().map(|v| {
        v.split(' ')
            .map(|v| v.parse::<usize>().unwrap())
            .collect::<Vec<usize>>()
    }) {
        sum += process_line(&line);
    }
    println!("{sum}");
}

fn process_line(line: &[usize]) -> usize {
    let tree = generate_lines(line);
    for item in tree {}
}

fn generate_lines(top: &[usize]) -> Vec<Vec<usize>> {
    let mut next = Vec::new();
    for (idx, item) in top.iter().enumerate() {
        if idx == 0 {
            continue;
        }
        next.push(item - top[idx - 1]);
    }
    if next.iter().all(|v| v == 0) {
        return;
    }
    generate_lines(&next)
}

fn extend_line(existing: &[usize], below: &[usize]) -> usize {
    if existing.iter().all(|v| *v == 0) {
        return 0;
    }
    let below_last = existing.len() - 2;
    let existing_last = existing.len() - 1;
    let last_existing = existing[existing_last];
    let last_below = below[below_last];
    last_existing + last_below
}
