use core::AocResult;

use crate::day4::utils::parse_pair;

use super::utils::{Pair, FILE_NAME};

const CORRECT_ANSWER: usize = 921;

pub fn solve() -> AocResult<()> {
    core::do_work(FILE_NAME, CORRECT_ANSWER, get_answer, |a, b| a == b)
}

fn get_answer(lines: Vec<String>) -> AocResult<usize> {
    Ok(lines.iter().map(parse_pair).filter(does_overlap).count())
}

fn does_overlap(pair: &Pair) -> bool {
    (pair.left.start <= pair.right.start && pair.left.end >= pair.right.start)
        || (pair.left.start >= pair.right.start && pair.left.start <= pair.right.end)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample() {
        let lines: Vec<String> = vec![
            String::from("2-4,6-8"),
            String::from("2-3,4-5"),
            String::from("5-7,7-9"),
            String::from("2-8,3-7"),
            String::from("6-6,4-6"),
            String::from("2-6,4-8"),
        ];

        assert_eq!(get_answer(lines).unwrap(), 4)
    }
}
