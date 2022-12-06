const CORRECT_ANSWER: u32 = 197400;

use crate::day1::utils;

pub fn solve() -> Result<(), String> {
    core::do_work(utils::FILE_NAME, CORRECT_ANSWER, get_answer, |a, b| a == b)
}

fn get_answer(lines: Vec<String>) -> u32 {
    let mut groups = utils::parse(lines);

    groups.sort_by(|a, b| b.cmp(a));
    groups.iter().take(3).sum::<u32>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample() {
        let lines: Vec<String> = vec![
            "1000".to_string(),
            "2000".to_string(),
            "3000".to_string(),
            "".to_string(),
            "4000".to_string(),
            "".to_string(),
            "5000".to_string(),
            "6000".to_string(),
            "".to_string(),
            "7000".to_string(),
            "8000".to_string(),
            "9000".to_string(),
            "".to_string(),
            "10000".to_string(),
        ];

        assert_eq!(get_answer(lines), 45000)
    }
}
