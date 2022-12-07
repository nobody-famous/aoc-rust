use super::utils::{get_char_set, priority, FILE_NAME};

const CORRECT_ANSWER: u32 = 8039;

pub fn solve() -> Result<(), String> {
    core::do_work(FILE_NAME, CORRECT_ANSWER, get_answer, |a, b| a == b)
}

fn get_answer(lines: Vec<String>) -> u32 {
    lines
        .iter()
        .map(|line| line.split_at(line.len() / 2))
        .map(find_same)
        .map(priority)
        .sum::<u32>()
}

fn find_same((a, b): (&str, &str)) -> char {
    let in_a = get_char_set(a);
    let in_b = get_char_set(b);

    in_a.iter().fold(' ', |acc, ch| {
        if acc != ' ' || !in_b.contains(ch) {
            acc
        } else {
            *ch
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample() {
        let lines = vec![
            "vJrwpWtwJgWrhcsFMMfFFhFp".to_string(),
            "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL".to_string(),
            "PmmdzqPrVvPwwTWBwg".to_string(),
            "wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn".to_string(),
            "ttgJtRGJQctTZtZT".to_string(),
            "CrZsJsPPZsGzwwsLwLmpwMDw".to_string(),
        ];

        assert_eq!(get_answer(lines), 157)
    }
}
