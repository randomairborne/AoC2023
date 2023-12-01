fn main() {
    let contents = std::fs::read_to_string("./input.txt").unwrap();
    let lines = contents.lines();
    let mut sum = 0;
    for line in lines {
        let chars = line.chars();
        for (idx, char) in chars.clone().enumerate() {
            let checking_slice = &line[..=idx];
            if char.is_ascii_digit() {
                sum += 10 * char.to_digit(10).unwrap();
                break;
            } else if let Some(num) = next_is_num(&checking_slice) {
                sum += 10 * num;
                break;
            }
        }
        for (idx, char) in chars.rev().enumerate() {
            let checking_slice = &line[line.len() - idx - 1..];
            if char.is_ascii_digit() {
                sum += char.to_digit(10).unwrap();
                break;
            } else if let Some(num) = next_is_num(checking_slice) {
                println!("{checking_slice}");
                sum += num;
                break;
            }
        }
    }
    println!("{sum}")
}

fn next_is_num(input: &str) -> Option<u32> {
    let int = if input.contains("one") {
        1
    } else if input.contains("two") {
        2
    } else if input.contains("three") {
        3
    } else if input.contains("four") {
        4
    } else if input.contains("five") {
        5
    } else if input.contains("six") {
        6
    } else if input.contains("seven") {
        7
    } else if input.contains("eight") {
        8
    } else if input.contains("nine") {
        9
    } else {
        return None;
    };
    Some(int)
}
