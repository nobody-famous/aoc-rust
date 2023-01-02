use crate::day3::utils::priority;

use super::utils::{get_char_set, FILE_NAME};

const CORRECT_ANSWER: u32 = 2510;

pub fn solve() -> Result<(), String> {
    core::do_work(FILE_NAME, CORRECT_ANSWER, get_answer, |a, b| a == b)
}

fn get_answer(lines: Vec<String>) -> Result<u32, String> {
    Ok(lines
        .iter()
        .fold(vec![vec![]], |mut acc, line| {
            if let Some(mut grp) = acc.pop() {
                if grp.len() < 3 {
                    grp.push(line);
                    acc.push(grp);
                } else {
                    acc.push(grp);
                    acc.push(vec![line]);
                }
            }

            acc
        })
        .iter()
        .map(find_badge)
        .map(priority)
        .sum::<u32>())
}

fn find_badge(group: &Vec<&String>) -> char {
    let a = get_char_set(group[0].as_str());
    let b = get_char_set(group[1].as_str());
    let c = get_char_set(group[2].as_str());

    a.iter().fold(' ', |acc, ch| {
        if acc != ' ' || !b.contains(ch) || !c.contains(ch) {
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

        assert_eq!(get_answer(lines), Ok(70))
    }
}
