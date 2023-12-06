fn main() {
    let input = std::fs::read_to_string("./input.txt").unwrap();
    let lines: Vec<String> = input.lines().map(ToString::to_string).collect();
    let times: Vec<usize> = lines[0]
        .trim_start_matches("Time:")
        .split(' ')
        .filter(|v| !v.trim().is_empty())
        .map(|v| v.trim().parse().unwrap())
        .collect();
    let distances: Vec<usize> = lines[1]
        .trim_start_matches("Distance:")
        .split(' ')
        .filter(|v| !v.trim().is_empty())
        .map(|v| v.trim().parse().unwrap())
        .collect();
    let pairs: Vec<(usize, usize)> = times.into_iter().zip(distances.into_iter()).collect();
    let mut total = 1;
    for (dur, dist) in pairs {
        total *= solves(dur, dist);
    }
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
