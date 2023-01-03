use super::utils::{create_visible, parse_rows, FILE_NAME};

const CORRECT_ANSWER: usize = 1543;

pub fn solve() -> Result<(), String> {
    core::do_work(FILE_NAME, CORRECT_ANSWER, get_answer, |a, b| a == b)
}

fn get_answer(lines: Vec<String>) -> Result<usize, String> {
    let rows = parse_rows(lines);
    let mut visible = create_visible(&rows);

    for idx in 1..rows.len() - 1 {
        check_row(&rows, &mut visible, idx);
    }

    for idx in 1..rows.len() - 1 {
        check_col(&rows, &mut visible, idx);
    }

    Ok(visible.iter().fold(0, |acc, row| {
        acc + row
            .iter()
            .fold(0, |acc, col| acc + if *col { 1 } else { 0 })
    }))
}

fn check_row(rows: &Vec<Vec<usize>>, visible: &mut Vec<Vec<bool>>, idx: usize) {
    let row = &rows[idx];
    let mut max = rows[idx][0];

    for col in 1..row.len() - 1 {
        max = update_row(row, visible, idx, col, max);
    }

    max = row[row.len() - 1];
    for col in (1..row.len() - 1).rev() {
        max = update_row(row, visible, idx, col, max);
    }
}

fn check_col(rows: &Vec<Vec<usize>>, visible: &mut Vec<Vec<bool>>, idx: usize) {
    let mut max = rows[0][idx];

    for row in 1..rows.len() - 1 {
        max = update_col(rows, visible, idx, row, max);
    }

    max = rows[rows.len() - 1][idx];
    for row in (1..rows.len() - 1).rev() {
        max = update_col(rows, visible, idx, row, max);
    }
}

fn update_col(
    rows: &Vec<Vec<usize>>,
    visible: &mut Vec<Vec<bool>>,
    idx: usize,
    row: usize,
    max: usize,
) -> usize {
    let tree = rows[row][idx];
    let mut new_max = max;

    visible[row][idx] = visible[row][idx] || tree > max;

    if tree > max {
        new_max = tree;
    }

    new_max
}

fn update_row(
    row: &Vec<usize>,
    visible: &mut Vec<Vec<bool>>,
    idx: usize,
    col: usize,
    max: usize,
) -> usize {
    let tree = row[col];
    let mut new_max = max;

    visible[idx][col] = visible[idx][col] || tree > max;

    if tree > max {
        new_max = tree;
    }

    new_max
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample() {
        let lines: Vec<String> = vec![
            "30373".to_string(),
            "25512".to_string(),
            "65332".to_string(),
            "33549".to_string(),
            "35390".to_string(),
        ];

        assert_eq!(get_answer(lines), Ok(21))
    }
}
