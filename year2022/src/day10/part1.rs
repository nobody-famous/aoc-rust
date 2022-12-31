use std::vec;

use super::utils::{exec, parse, FILE_NAME};

const CORRECT_ANSWER: isize = 0;

pub fn solve() -> Result<(), String> {
    core::do_work(FILE_NAME, CORRECT_ANSWER, get_answer, |a, b| a == b)
}

fn get_answer(lines: Vec<String>) -> isize {
    let ops = parse(lines);
    let mut strength: isize = 0;

    let proc = |cycle, x| match cycle {
        20 | 60 | 100 | 140 | 180 | 220 => {
            strength += cycle * x;
        }
        _ => (),
    };

    exec(&ops, proc);

    strength
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample() {
        let lines: Vec<String> = vec![
            "addx 15".to_string(),
            "addx -11".to_string(),
            "addx 6".to_string(),
            "addx -3".to_string(),
            "addx 5".to_string(),
            "addx -1".to_string(),
            "addx -8".to_string(),
            "addx 13".to_string(),
            "addx 4".to_string(),
            "noop".to_string(),
            "addx -1".to_string(),
            "addx 5".to_string(),
            "addx -1".to_string(),
            "addx 5".to_string(),
            "addx -1".to_string(),
            "addx 5".to_string(),
            "addx -1".to_string(),
            "addx 5".to_string(),
            "addx -1".to_string(),
            "addx -35".to_string(),
            "addx 1".to_string(),
            "addx 24".to_string(),
            "addx -19".to_string(),
            "addx 1".to_string(),
            "addx 16".to_string(),
            "addx -11".to_string(),
            "noop".to_string(),
            "noop".to_string(),
            "addx 21".to_string(),
            "addx -15".to_string(),
            "noop".to_string(),
            "noop".to_string(),
            "addx -3".to_string(),
            "addx 9".to_string(),
            "addx 1".to_string(),
            "addx -3".to_string(),
            "addx 8".to_string(),
            "addx 1".to_string(),
            "addx 5".to_string(),
            "noop".to_string(),
            "noop".to_string(),
            "noop".to_string(),
            "noop".to_string(),
            "noop".to_string(),
            "addx -36".to_string(),
            "noop".to_string(),
            "addx 1".to_string(),
            "addx 7".to_string(),
            "noop".to_string(),
            "noop".to_string(),
            "noop".to_string(),
            "addx 2".to_string(),
            "addx 6".to_string(),
            "noop".to_string(),
            "noop".to_string(),
            "noop".to_string(),
            "noop".to_string(),
            "noop".to_string(),
            "addx 1".to_string(),
            "noop".to_string(),
            "noop".to_string(),
            "addx 7".to_string(),
            "addx 1".to_string(),
            "noop".to_string(),
            "addx -13".to_string(),
            "addx 13".to_string(),
            "addx 7".to_string(),
            "noop".to_string(),
            "addx 1".to_string(),
            "addx -33".to_string(),
            "noop".to_string(),
            "noop".to_string(),
            "noop".to_string(),
            "addx 2".to_string(),
            "noop".to_string(),
            "noop".to_string(),
            "noop".to_string(),
            "addx 8".to_string(),
            "noop".to_string(),
            "addx -1".to_string(),
            "addx 2".to_string(),
            "addx 1".to_string(),
            "noop".to_string(),
            "addx 17".to_string(),
            "addx -9".to_string(),
            "addx 1".to_string(),
            "addx 1".to_string(),
            "addx -3".to_string(),
            "addx 11".to_string(),
            "noop".to_string(),
            "noop".to_string(),
            "addx 1".to_string(),
            "noop".to_string(),
            "addx 1".to_string(),
            "noop".to_string(),
            "noop".to_string(),
            "addx -13".to_string(),
            "addx -19".to_string(),
            "addx 1".to_string(),
            "addx 3".to_string(),
            "addx 26".to_string(),
            "addx -30".to_string(),
            "addx 12".to_string(),
            "addx -1".to_string(),
            "addx 3".to_string(),
            "addx 1".to_string(),
            "noop".to_string(),
            "noop".to_string(),
            "noop".to_string(),
            "addx -9".to_string(),
            "addx 18".to_string(),
            "addx 1".to_string(),
            "addx 2".to_string(),
            "noop".to_string(),
            "noop".to_string(),
            "addx 9".to_string(),
            "noop".to_string(),
            "noop".to_string(),
            "noop".to_string(),
            "addx -1".to_string(),
            "addx 2".to_string(),
            "addx -37".to_string(),
            "addx 1".to_string(),
            "addx 3".to_string(),
            "noop".to_string(),
            "addx 15".to_string(),
            "addx -21".to_string(),
            "addx 22".to_string(),
            "addx -6".to_string(),
            "addx 1".to_string(),
            "noop".to_string(),
            "addx 2".to_string(),
            "addx 1".to_string(),
            "noop".to_string(),
            "addx -10".to_string(),
            "noop".to_string(),
            "noop".to_string(),
            "addx 20".to_string(),
            "addx 1".to_string(),
            "addx 2".to_string(),
            "addx 2".to_string(),
            "addx -6".to_string(),
            "addx -11".to_string(),
            "noop".to_string(),
            "noop".to_string(),
            "noop".to_string(),
        ];

        assert_eq!(get_answer(lines), 13140)
    }
}
