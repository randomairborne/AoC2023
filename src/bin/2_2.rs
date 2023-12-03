fn main() {
    let contents = std::fs::read_to_string("./input.txt").unwrap();
    let lines = contents.lines();
    let mut sum = 0;
    for line in lines {
        let game_line: Vec<&str> = line.splitn(2, ':').collect();
        let rounds = game_line[1].split(';');
        let mut max_red = 0;
        let mut max_green = 0;
        let mut max_blue = 0;
        for round in rounds {
            let items: Vec<&str> = round.split(',').collect();
            let red = find_color(&items, "red").unwrap_or(1);
            let green = find_color(&items, "green").unwrap_or(1);
            let blue = find_color(&items, "blue").unwrap_or(1);
            if red > max_red {
                max_red = red;
            }
            if green > max_green {
                max_green = green;
            }
            if blue > max_blue {
                max_blue = blue;
            }
        }
        sum += max_red * max_green * max_blue;
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
