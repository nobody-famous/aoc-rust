use super::utils::{FILE_NAME, parse};

const CORRECT_ANSWER: usize = 0;

pub fn solve() -> Result<(), String> {
    core::do_work(FILE_NAME, CORRECT_ANSWER, get_answer, |a, b| a == b)
}

fn get_answer(lines: Vec<String>) -> Result<usize, String> {
    parse(lines)?;
    Err(String::from("get_answer Not Done Yet"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample() {
        let lines = vec![
            "Blueprint 1:".to_string(),
            "  Each ore robot costs 4 ore.".to_string(),
            "  Each clay robot costs 2 ore.".to_string(),
            "  Each obsidian robot costs 3 ore and 14 clay.".to_string(),
            "  Each geode robot costs 2 ore and 7 obsidian.".to_string(),
            "".to_string(),
            "Blueprint 2:".to_string(),
            "  Each ore robot costs 2 ore.".to_string(),
            "  Each clay robot costs 3 ore.".to_string(),
            "  Each obsidian robot costs 3 ore and 8 clay.".to_string(),
            "  Each geode robot costs 3 ore and 12 obsidian.".to_string(),
        ];

        assert_eq!(get_answer(lines), Ok(33))
    }
}
