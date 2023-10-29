use core::AocResult;

use super::utils::FILE_NAME;
use crate::day6::utils::get_answer;

const CORRECT_ANSWER: u32 = 3697;
const MARKER_LENGTH: usize = 14;

pub fn solve() -> AocResult<()> {
    core::do_work(
        FILE_NAME,
        CORRECT_ANSWER,
        |lines| get_answer(lines, MARKER_LENGTH),
        |a, b| a == b,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_1() {
        let lines: Vec<String> = vec![String::from("bvwbjplbgvbhsrlpgdmjqwftvncz")];

        assert_eq!(get_answer(lines, MARKER_LENGTH).unwrap(), 23)
    }

    #[test]
    fn sample_2() {
        let lines: Vec<String> = vec![String::from("nppdvjthqldpwncqszvftbrmjlhg")];

        assert_eq!(get_answer(lines, MARKER_LENGTH).unwrap(), 23)
    }

    #[test]
    fn sample_3() {
        let lines: Vec<String> = vec![String::from("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg")];

        assert_eq!(get_answer(lines, MARKER_LENGTH).unwrap(), 29)
    }

    #[test]
    fn sample_4() {
        let lines: Vec<String> = vec![String::from("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw")];

        assert_eq!(get_answer(lines, MARKER_LENGTH).unwrap(), 26)
    }
}
