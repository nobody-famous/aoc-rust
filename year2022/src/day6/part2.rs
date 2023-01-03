use super::utils::FILE_NAME;
use crate::day6::utils::get_answer;

const CORRECT_ANSWER: u32 = 3697;
const MARKER_LENGTH: usize = 14;

pub fn solve() -> Result<(), String> {
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
        let lines: Vec<String> = vec!["bvwbjplbgvbhsrlpgdmjqwftvncz".to_string()];

        assert_eq!(get_answer(lines, MARKER_LENGTH), Ok(23))
    }

    #[test]
    fn sample_2() {
        let lines: Vec<String> = vec!["nppdvjthqldpwncqszvftbrmjlhg".to_string()];

        assert_eq!(get_answer(lines, MARKER_LENGTH), Ok(23))
    }

    #[test]
    fn sample_3() {
        let lines: Vec<String> = vec!["nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg".to_string()];

        assert_eq!(get_answer(lines, MARKER_LENGTH), Ok(29))
    }

    #[test]
    fn sample_4() {
        let lines: Vec<String> = vec!["zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw".to_string()];

        assert_eq!(get_answer(lines, MARKER_LENGTH), Ok(26))
    }
}
