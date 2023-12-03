const MAX_RED: usize = 12;
const MAX_GREEN: usize = 13;
const MAX_BLUE: usize = 14;

fn main() {
    let contents = std::fs::read_to_string("./input.txt").unwrap();
    let lines = contents.lines();
    let mut sum = 0;
    'lines: for line in lines {
        let game_line: Vec<&str> = line.splitn(2, ':').collect();
        let game: usize = game_line[0].strip_prefix("Game ").unwrap().parse().unwrap();
        let rounds = game_line[1].split(';');
        for round in rounds {
            let items: Vec<&str> = round.split(',').collect();
            if find_color(&items, "green")
                .map(|v| v > MAX_GREEN)
                .unwrap_or(false)
                || find_color(&items, "red")
                    .map(|v| v > MAX_RED)
                    .unwrap_or(false)
                || find_color(&items, "blue")
                    .map(|v| v > MAX_BLUE)
                    .unwrap_or(false)
            {
                continue 'lines;
            }
        }
        sum += game;
    }
    println!("{sum}")
}

fn find_color(items: &[&str], color: &str) -> Option<usize> {
    Some(
        items
            .iter()
            .find(|v| v.contains(color))?
            .trim_end_matches(color)
            .trim()
            .parse()
            .unwrap(),
    )
}
