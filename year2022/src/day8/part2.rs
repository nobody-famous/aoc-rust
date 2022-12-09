use super::utils::{parse_rows, FILE_NAME};

const CORRECT_ANSWER: usize = 0;

pub fn solve() -> Result<(), String> {
    core::do_work(FILE_NAME, CORRECT_ANSWER, get_answer, |a, b| a == b)
}

fn get_answer(lines: Vec<String>) -> usize {
    let rows = parse_rows(lines);
    let mut scores: Vec<Vec<usize>> = rows
        .clone()
        .iter()
        .map(|row| row.clone().iter().map(|_| 0).collect())
        .collect();

    for row in 1..rows.len() - 1 {
        for col in 1..rows[0].len() - 1 {
            println!("{},{}", row, col);
        }
    }

    println!("SCORES {:?}", scores);

    0
}

fn get_score(rows: &Vec<Vec<usize>>, row: usize, col: usize) -> usize {
    0
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
