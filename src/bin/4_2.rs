fn main() {
    let contents = std::fs::read_to_string("./input.txt").unwrap();
    let lines: Vec<&str> = contents.lines().collect();
    let mut sum = 0;
    for (idx, _) in lines.iter().enumerate() {
        sum += process_card(&lines, idx)
    }
    println!("{sum}");
}

fn process_card(data: &[&str], idx: usize) -> usize {
    let mut matches = 0;
    let line: Vec<&str> = data[idx].split(':').nth(1).unwrap().split('|').collect();
    let winners = parse(line[0]);
    let values = parse(line[1]);
    for value in &values {
        if winners.contains(value) {
            matches += 1;
        }
    }
    let mut cards = 1;
    for start_idx in idx + 1..=idx + matches {
        cards += process_card(data, start_idx);
    }
    cards
}

fn parse(input: &str) -> Vec<usize> {
    let mut output = Vec::new();
    for item in input.split(' ') {
        if item.chars().all(|v| v.is_ascii_whitespace()) {
            continue;
        }
        output.push(item.parse().unwrap())
    }
    output
}
