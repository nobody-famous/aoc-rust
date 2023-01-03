const CORRECT_ANSWER: u32 = 69206;

use crate::day1::utils;

pub fn solve() -> Result<(), String> {
    core::do_work(utils::FILE_NAME, CORRECT_ANSWER, get_answer, |a, b| a == b)
}

fn get_answer(lines: Vec<String>) -> Result<u32, String> {
    let groups = utils::parse(lines);

    match groups.iter().max() {
        Some(num) => Ok(*num),
        None => Err(String::from("No max found")),
    }
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

        assert_eq!(get_answer(lines), Ok(24000))
    }
}
