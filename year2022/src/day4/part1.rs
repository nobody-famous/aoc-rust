use core::AocResult;

use crate::day4::utils::parse_pair;

use super::utils::Pair;

pub fn solve(file_name: &str) -> AocResult<usize> {
    core::do_work(file_name, get_answer)
}

fn get_answer(lines: Vec<String>) -> AocResult<usize> {
    Ok(lines.iter().map(|line| parse_pair(line)).filter(does_overlap).count())
}

fn does_overlap(pair: &Pair) -> bool {
    (pair.left.start <= pair.right.start && pair.left.end >= pair.right.end)
        || (pair.left.start >= pair.right.start && pair.left.end <= pair.right.end)
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

        assert_eq!(get_answer(lines).unwrap(), 2)
    }
}
