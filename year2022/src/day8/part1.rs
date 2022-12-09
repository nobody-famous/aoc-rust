use super::utils::FILE_NAME;

const CORRECT_ANSWER: usize = 1543;

pub fn solve() -> Result<(), String> {
    core::do_work(FILE_NAME, CORRECT_ANSWER, get_answer, |a, b| a == b)
}

fn get_answer(lines: Vec<String>) -> usize {
    let rows: Vec<Vec<usize>> = lines.iter().map(|line| parse_row(line)).collect();
    let mut visible: Vec<Vec<bool>> = rows
        .clone()
        .iter()
        .map(|row| row.clone().iter().map(|_| false).collect())
        .collect();

    set_boundary(&mut visible);

    for idx in 1..rows.len() - 1 {
        check_row(&rows, &mut visible, idx);
    }

    for idx in 1..rows.len() - 1 {
        check_col(&rows, &mut visible, idx);
    }

    visible.iter().fold(0, |acc, row| {
        acc + row
            .iter()
            .fold(0, |acc, col| acc + if *col { 1 } else { 0 })
    })
}

fn set_boundary(visible: &mut Vec<Vec<bool>>) {
    let idx = visible.len() - 1;

    visible[0] = visible[0].iter().map(|_| true).collect();
    visible[idx] = visible[idx].iter().map(|_| true).collect();

    for idx in 1..visible.len() - 1 {
        let end = visible[idx].len() - 1;
        visible[idx][0] = true;
        visible[idx][end] = true;
    }
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

fn parse_row(line: &str) -> Vec<usize> {
    line.chars().fold(vec![], |mut acc, ch| {
        acc.push(ch as usize - '0' as usize);
        acc
    })
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

        assert_eq!(get_answer(lines), 21)
    }
}
