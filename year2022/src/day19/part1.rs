use crate::day19::utils::State;

use super::utils::{parse, Blueprint, FILE_NAME};

const CORRECT_ANSWER: usize = 0;

pub fn solve() -> Result<(), String> {
    core::do_work(FILE_NAME, CORRECT_ANSWER, get_answer, |a, b| a == b)
}

fn get_answer(lines: Vec<String>) -> Result<usize, String> {
    let blueprints = parse(lines)?;
    let mut state = State::new();

    for _ in 0..2 {
        state = minute(&blueprints, &mut state);
        println!("STATE {:?}", state);
    }

    Err(String::from("get_answer Not Done Yet"))
}

fn minute(blueprints: &Vec<Blueprint>, state: &mut State) -> State {
    let new_ore = state.ore + state.ore_robots;
    let new_clay = state.clay + state.clay_robots;
    let new_obsidian = state.obsidian + state.obsidian_robots;

    State {
        ore: new_ore,
        clay: new_clay,
        obsidian: new_obsidian,
        ore_robots: state.ore_robots,
        clay_robots: state.clay_robots,
        obsidian_robots: state.obsidian_robots,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample() {
        let lines = vec![
            "Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.".to_string(),
            "Blueprint 2: Each ore robot costs 2 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 8 clay. Each geode robot costs 3 ore and 12 obsidian.".to_string(),
        ];

        assert_eq!(get_answer(lines), Ok(33))
    }
}
