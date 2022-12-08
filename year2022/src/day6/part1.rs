use super::utils::FILE_NAME;

const CORRECT_ANSWER: u32 = 1707;
const MARKER_LENGTH: usize = 4;

pub fn solve() -> Result<(), String> {
    core::do_work(
        FILE_NAME,
        CORRECT_ANSWER,
        |lines| get_answer(lines, MARKER_LENGTH),
        |a, b| a == b,
    )
}

fn get_answer(lines: Vec<String>, length: usize) -> u32 {
    let line: &String = &lines[0];
    let mut idx = length - 1;

    while idx < line.len() {
        match find_dupe(line, idx, MARKER_LENGTH) {
            Some(n) => idx = n + length + 1,
            None => return idx as u32,
        }
    }

    0
}

fn find_dupe(line: &String, start: usize, count: usize) -> Option<usize> {
    let bytes = line.as_bytes();
    let mut flags: usize = 0;

    for idx in ((start + 1) - count..(start + 1)).rev() {
        let ch = bytes[idx] as char;
        let shift = ch as usize - 'a' as usize;
        let mask = 1 << shift;
        let prev = flags & mask;

        flags |= mask;

        if prev != 0 {
            return Some(idx);
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_1() {
        let lines: Vec<String> = vec!["bvwbjplbgvbhsrlpgdmjqwftvncz".to_string()];

        assert_eq!(get_answer(lines, MARKER_LENGTH), 5)
    }

    #[test]
    fn sample_2() {
        let lines: Vec<String> = vec!["nppdvjthqldpwncqszvftbrmjlhg".to_string()];

        assert_eq!(get_answer(lines, MARKER_LENGTH), 6)
    }

    #[test]
    fn sample_3() {
        let lines: Vec<String> = vec!["nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg".to_string()];

        assert_eq!(get_answer(lines, MARKER_LENGTH), 10)
    }

    #[test]
    fn sample_4() {
        let lines: Vec<String> = vec!["zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw".to_string()];

        assert_eq!(get_answer(lines, MARKER_LENGTH), 11)
    }
}
