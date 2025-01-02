use core::AocResult;

use super::utils;

const CORRECT_ANSWER: u32 = 11150;

pub fn solve() -> AocResult<()> {
    core::do_work(utils::FILE_NAME, CORRECT_ANSWER, get_answer, |a, b| a == b)
}

fn get_answer(lines: Vec<String>) -> AocResult<u32> {
    Ok(lines.iter().map(|line| round_score(line)).sum::<u32>())
}

fn round_score(line: &str) -> u32 {
    match line {
        "A X" => 1 + 3,
        "A Y" => 2 + 6,
        "A Z" => 3,
        "B X" => 1,
        "B Y" => 2 + 3,
        "B Z" => 3 + 6,
        "C X" => 1 + 6,
        "C Y" => 2,
        "C Z" => 3 + 3,
        _ => 0,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample() {
        let lines: Vec<String> = vec![
            String::from("A Y"),
            String::from("B X"),
            String::from("C Z"),
        ];

        assert_eq!(get_answer(lines).unwrap(), 15)
    }
}
