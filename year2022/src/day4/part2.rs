use crate::{day4::utils::parse_pair, utils::do_work};

use super::utils::{Pair, FILE_NAME};

const CORRECT_ANSWER: usize = 0;

pub fn solve() -> Result<(), String> {
    do_work(FILE_NAME, CORRECT_ANSWER, get_answer, |a, b| a == b)
}

fn get_answer(lines: Vec<String>) -> usize {
    lines.iter().map(parse_pair).filter(does_overlap).count()
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
            "2-4,6-8".to_string(),
            "2-3,4-5".to_string(),
            "5-7,7-9".to_string(),
            "2-8,3-7".to_string(),
            "6-6,4-6".to_string(),
            "2-6,4-8".to_string(),
        ];

        assert_eq!(get_answer(lines), 4)
    }
}
