use core::AocResult;

use super::utils::{get_char_set, priority, FILE_NAME};

pub fn solve() -> AocResult<u32> {
    core::do_work(FILE_NAME, get_answer)
}

fn get_answer(lines: Vec<String>) -> AocResult<u32> {
    Ok(lines
        .iter()
        .map(|line| line.split_at(line.len() / 2))
        .map(find_same)
        .map(priority)
        .sum::<u32>())
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
            String::from("vJrwpWtwJgWrhcsFMMfFFhFp"),
            String::from("jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL"),
            String::from("PmmdzqPrVvPwwTWBwg"),
            String::from("wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn"),
            String::from("ttgJtRGJQctTZtZT"),
            String::from("CrZsJsPPZsGzwwsLwLmpwMDw"),
        ];

        assert_eq!(get_answer(lines).unwrap(), 157)
    }
}
