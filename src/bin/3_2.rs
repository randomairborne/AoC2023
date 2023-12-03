fn main() {
    let contents = std::fs::read_to_string("./input.txt").unwrap();
    let lines: Vec<Vec<char>> = contents.lines().map(|v| v.chars().collect()).collect();
    let mut output_tbl = Vec::with_capacity(lines.len());
    let rows = lines.len();
    let columns = lines[0].len();
    for _ in 0..rows {
        output_tbl.push(vec![None; columns])
    }
    let mut lookups: Vec<usize> = Vec::new();
    let mut next_num = String::with_capacity(3);
    let mut sum = 0;
    for (row_idx, line) in lines.iter().enumerate() {
        for (column_idx, item) in line.iter().enumerate() {
            process_item(
                &mut next_num,
                &mut lookups,
                &mut output_tbl,
                row_idx,
                column_idx,
                *item,
            );
        }
    }
    for (row_idx, line) in lines.iter().enumerate() {
        for (column_idx, item) in line.iter().enumerate() {
            if *item == '*' {
                sum += find_parts(&output_tbl, &lookups, row_idx, column_idx, rows, columns);
            }
        }
    }
    println!("{sum}");
}

fn process_item(
    next_num: &mut String,
    lookups: &mut Vec<usize>,
    tbl: &mut Vec<Vec<Option<usize>>>,
    row_idx: usize,
    column_idx: usize,
    item: char,
) {
    if item.is_ascii_digit() {
        next_num.push(item)
    } else if !next_num.is_empty() {
        set_cell(next_num, lookups, tbl, row_idx, column_idx)
    }
    if column_idx == tbl[0].len() - 1 && !next_num.is_empty() {
        set_cell(next_num, lookups, tbl, row_idx, column_idx)
    }
}

fn set_cell(
    next_num: &mut String,
    lookups: &mut Vec<usize>,
    tbl: &mut Vec<Vec<Option<usize>>>,
    row_idx: usize,
    column_idx: usize,
) {
    let num: usize = next_num.parse().unwrap();
    let string_offset = next_num.len() - 1;
    lookups.push(num);
    for idx in column_idx - string_offset - 1..column_idx {
        tbl[row_idx][idx] = Some(lookups.len() - 1);
    }
    *next_num = String::with_capacity(3);
}

fn find_parts(
    tbl: &Vec<Vec<Option<usize>>>,
    lookups: &Vec<usize>,
    row_idx: usize,
    column_idx: usize,
    rows: usize,
    columns: usize,
) -> usize {
    let min_column_idx = column_idx.saturating_sub(1);
    let max_column_idx = (column_idx + 1).clamp(0, columns - 1);
    let min_row_idx = row_idx.saturating_sub(1);
    let max_row_idx = (row_idx + 1).clamp(0, rows - 1);
    let mut first = None;
    let mut seen = Vec::new();
    for idx in min_row_idx..=max_row_idx {
        let parts = &tbl[idx][min_column_idx..=max_column_idx];
        for part_idx in parts {
            if let Some(part_idx) = part_idx.clone() {
                if seen.contains(&part_idx) {
                    continue;
                }
                seen.push(part_idx);
                if let Some(val) = first {
                    return lookups[part_idx] * val;
                } else {
                    first = Some(lookups[part_idx]);
                }
            }
        }
    }
    0
}
