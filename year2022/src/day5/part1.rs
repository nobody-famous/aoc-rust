const CORRECT_ANSWER: &str = "";

use crate::{day5::utils::parse, day5::utils::FILE_NAME};

pub fn solve() -> Result<(), String> {
    core::do_work(
        FILE_NAME,
        String::from(CORRECT_ANSWER),
        get_answer,
        |a, b| a.eq(b),
    )
}

fn get_answer(lines: Vec<String>) -> String {
    parse(lines);
    "".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample() {
        let lines: Vec<String> = vec![
            "[D]    ".to_string(),
            "[N] [C]    ".to_string(),
            "[Z] [M] [P]".to_string(),
            " 1   2   3 ".to_string(),
            "".to_string(),
            "move 1 from 2 to 1".to_string(),
            "move 3 from 1 to 3".to_string(),
            "move 2 from 2 to 1".to_string(),
            "move 1 from 1 to 2".to_string(),
        ];

        assert_eq!(get_answer(lines), String::from("CMZ"))
    }
}
