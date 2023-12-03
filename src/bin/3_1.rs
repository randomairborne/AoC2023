fn main() {
    let contents = std::fs::read_to_string("./input.txt").unwrap();
    let lines: Vec<Vec<char>> = contents.lines().map(|v| v.chars().collect()).collect();
    let lines_slices: Vec<&[char]> = lines.iter().map(|v| v.as_slice()).collect();
    let mut sum = 0;
    let mut next_num = String::with_capacity(3);
    for (row_idx, line) in lines.iter().enumerate() {
        for (column_idx, item) in line.iter().enumerate() {
            sum += process_item(&mut next_num, &lines_slices, row_idx, column_idx, *item);
        }
    }
    println!("{sum}");
}

fn process_item(
    next_num: &mut String,
    tbl: &[&[char]],
    row_idx: usize,
    column_idx: usize,
    item: char,
) -> usize {
    if item.is_ascii_digit() {
        next_num.push(item)
    } else if !next_num.is_empty() {
        let num: usize = next_num.parse().unwrap();
        let string_len = next_num.len();
        *next_num = String::with_capacity(3);
        if check_table(tbl, row_idx, column_idx - string_len, string_len) {
            return num;
        }
    }
    if column_idx == tbl[0].len() - 1 && !next_num.is_empty() {
        let num: usize = next_num.parse().unwrap();
        let string_len = next_num.len();
        *next_num = String::with_capacity(3);
        if check_table(tbl, row_idx, column_idx - string_len, string_len) {
            return num;
        }
    }
    0
}

fn check_table(tbl: &[&[char]], row_idx: usize, column_idx: usize, len: usize) -> bool {
    let min_column_idx = column_idx.saturating_sub(1);
    let max_column_idx = (column_idx + len).clamp(0, tbl[0].len() - 1);
    let min_row_idx = row_idx.saturating_sub(1);
    let max_row_idx = (row_idx + 1).clamp(0, tbl.len() - 1);
    for idx in min_row_idx..=max_row_idx {
        let chars = &tbl[idx][min_column_idx..=max_column_idx];
        for item in chars {
            if !item.is_ascii_alphanumeric() && *item != '.' {
                return true;
            }
        }
    }
    false
}
