use aoc::ProblemResult;

use super::utils::{get_mask, parse, walk, FILE_NAME};

const CORRECT_ANSWER: usize = 1716;

pub fn solve() -> ProblemResult<()> {
    aoc::do_work(FILE_NAME, CORRECT_ANSWER, get_answer, |a, b| a == b)
}

fn get_answer(lines: Vec<String>) -> ProblemResult<usize> {
    let config = parse(lines)?;
    let start = get_mask(&config.masks, &"AA".to_string())?;
    let flows = walk(&config, start, 30)?;

    let dists: Vec<usize> = flows.iter().map(|(_, value)| *value).collect();

    match dists.iter().max() {
        Some(v) => Ok(*v),
        None => Err("No max found".into()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample() {
        let lines = vec![
            String::from("Valve AA has flow rate=0; tunnels lead to valves DD, II, BB"),
            String::from("Valve BB has flow rate=13; tunnels lead to valves CC, AA"),
            String::from("Valve CC has flow rate=2; tunnels lead to valves DD, BB"),
            String::from("Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE"),
            String::from("Valve EE has flow rate=3; tunnels lead to valves FF, DD"),
            String::from("Valve FF has flow rate=0; tunnels lead to valves EE, GG"),
            String::from("Valve GG has flow rate=0; tunnels lead to valves FF, HH"),
            String::from("Valve HH has flow rate=22; tunnel leads to valve GG"),
            String::from("Valve II has flow rate=0; tunnels lead to valves AA, JJ"),
            String::from("Valve JJ has flow rate=21; tunnel leads to valve II"),
        ];

        assert_eq!(get_answer(lines).unwrap(), 1651)
    }
}
