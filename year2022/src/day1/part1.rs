const CORRECT_ANSWER: u32 = 69206;

use aoc::ProblemResult;

use crate::day1::utils;

pub fn solve() -> ProblemResult<()> {
    aoc::do_work(utils::FILE_NAME, CORRECT_ANSWER, get_answer, |a, b| a == b)
}

fn get_answer(lines: Vec<String>) -> ProblemResult<u32> {
    let groups = utils::parse(lines);

    match groups.iter().max() {
        Some(num) => Ok(*num),
        None => Err("No max found".into()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample() {
        let lines: Vec<String> = vec![
            String::from("1000"),
            String::from("2000"),
            String::from("3000"),
            String::from(""),
            String::from("4000"),
            String::from(""),
            String::from("5000"),
            String::from("6000"),
            String::from(""),
            String::from("7000"),
            String::from("8000"),
            String::from("9000"),
            String::from(""),
            String::from("10000"),
        ];

        assert_eq!(get_answer(lines).unwrap(), 24000)
    }
}
