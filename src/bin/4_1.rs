fn main() {
    let contents = std::fs::read_to_string("./input.txt").unwrap();
    let mut sum = 0;
    for line in contents.lines() {
        let mut score = 0;
        let line: Vec<&str> = line.split(':').nth(1).unwrap().split('|').collect();
        let winners = parse(line[0]);
        let values = parse(line[1]);
        for value in values {
            if winners.contains(&value) {
                if score == 0 {
                    score += 1;
                } else {
                    score *= 2;
                }
            }
        }
        sum += score;
    }
    println!("{sum}");
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
