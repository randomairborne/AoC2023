fn main() {
    let contents = std::fs::read_to_string("./input.txt").unwrap();
    let lines = contents.lines();
    let mut sum = 0;
    for line in lines {
        let chars = line.chars();
        for char in chars.clone() {
            if char.is_ascii_digit() {
                sum += 10 * char.to_digit(10).unwrap();
                break;
            }
        }
        for char in chars.rev() {
            if char.is_ascii_digit() {
                sum += char.to_digit(10).unwrap();
                break;
            }
        }
    }
    println!("{sum}")
}
