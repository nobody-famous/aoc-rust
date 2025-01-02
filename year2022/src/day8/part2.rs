use core::AocResult;

use super::utils::{parse_rows, FILE_NAME};

const CORRECT_ANSWER: usize = 595080;

pub fn solve() -> AocResult<()> {
    core::do_work(FILE_NAME, CORRECT_ANSWER, get_answer, |a, b| a == b)
}

fn get_answer(lines: Vec<String>) -> AocResult<usize> {
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

    Ok(high)
}

fn get_score(rows: &[Vec<usize>], row: usize, col: usize) -> usize {
    let target = rows[row][col];

    let scan_v = |is_done: &mut bool, item: &Vec<usize>| {
        if *is_done {
            None
        } else {
            *is_done = item[col] >= target;
            Some(0)
        }
    };
    let scan_h = |is_done: &mut bool, item: &usize| {
        if *is_done {
            None
        } else {
            *is_done = *item >= target;
            Some(0)
        }
    };

    let north = rows.iter().take(row).rev().scan(false, scan_v).count();
    let south = rows.iter().skip(row + 1).scan(false, scan_v).count();
    let west = rows[row].iter().take(col).rev().scan(false, scan_h).count();
    let east = rows[row].iter().skip(col + 1).scan(false, scan_h).count();

    north * south * east * west
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample() {
        let lines: Vec<String> = vec![
            String::from("30373"),
            String::from("25512"),
            String::from("65332"),
            String::from("33549"),
            String::from("35390"),
        ];

        assert_eq!(get_answer(lines).unwrap(), 8)
    }
}
