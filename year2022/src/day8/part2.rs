use super::utils::{parse_rows, FILE_NAME};

const CORRECT_ANSWER: usize = 595080;

pub fn solve() -> Result<(), String> {
    core::do_work(FILE_NAME, CORRECT_ANSWER, get_answer, |a, b| a == b)
}

fn get_answer(lines: Vec<String>) -> usize {
    let rows = parse_rows(lines);
    let mut high: usize = 0;

    for row in 1..rows.len() - 1 {
        for col in 1..rows[0].len() - 1 {
            let score = get_score(&rows, row, col);
            if score > high {
                high = score;
            }
        }
    }

    high
}

fn get_score(rows: &Vec<Vec<usize>>, row: usize, col: usize) -> usize {
    let target = rows[row][col];
    let mut north = 0;
    let mut south = 0;
    let mut east = 0;
    let mut west = 0;

    for idx in (0..row).rev() {
        north += 1;
        if rows[idx][col] >= target {
            break;
        }
    }

    for idx in row + 1..rows.len() {
        south += 1;
        if rows[idx][col] >= target {
            break;
        }
    }

    for idx in (0..col).rev() {
        west += 1;
        if rows[row][idx] >= target {
            break;
        }
    }

    for idx in col + 1..rows[0].len() {
        east += 1;
        if rows[row][idx] >= target {
            break;
        }
    }

    north * south * east * west
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

        assert_eq!(get_answer(lines), 8)
    }
}
