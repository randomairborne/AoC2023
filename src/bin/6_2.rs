fn main() {
    let input = std::fs::read_to_string("./input.txt").unwrap();
    let lines: Vec<String> = input.lines().map(ToString::to_string).collect();
    let duration_str: String = lines[0]
        .trim_start_matches("Distance:")
        .chars()
        .filter(|v| v.is_ascii_digit())
        .collect();
    let distance_str: String = lines[1]
        .trim_start_matches("Distance:")
        .chars()
        .filter(|v| v.is_ascii_digit())
        .collect();
    let dur: usize = duration_str.parse().unwrap();
    let dist: usize = distance_str.parse().unwrap();
    let total = solves(dur, dist);
    println!("{total:?}")
}

fn solves(dur: usize, dist_needed: usize) -> usize {
    let mut ways = 0;
    for hold_time in 1..dur {
        let move_time = dur - hold_time;
        let distance = move_time * hold_time;
        if distance > dist_needed {
            ways += 1;
        }
    }
    ways
}
