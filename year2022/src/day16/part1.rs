use crate::day16::utils::State;

use super::utils::{parse, ToVisit, Valve, FILE_NAME};

const CORRECT_ANSWER: usize = 0;

pub fn solve() -> Result<(), String> {
    core::do_work(FILE_NAME, CORRECT_ANSWER, get_answer, |a, b| a == b)
}

fn get_answer(lines: Vec<String>) -> usize {
    let valves = parse(lines);
    let mut state = State::new(valves);

    for key in state.valves.keys() {
        let valve = &state.valves[key];

        if valve.rate == 0 {
            state.opened.insert(key.to_string());
        }
    }

    let mut nodes = vec![ToVisit::new(state, "AA".to_string())];

    for _ in 1..30 {
        nodes = walk(&nodes);
        println!("NODES: {}", nodes.len());
    }

    0
}

fn walk(to_visit: &Vec<ToVisit>) -> Vec<ToVisit> {
    let nodes = to_visit.iter().map(|n| n);
    let mut new_nodes: Vec<ToVisit> = vec![];

    for node in nodes {
        let valve: &Valve = &node.state.valves[&node.name];
        let mut new_state = node.state.clone();

        new_state.minutes += 1;

        let mut kid_nodes: Vec<ToVisit> = valve
            .kids
            .iter()
            .map(|k| ToVisit::new(new_state.clone(), k.clone()))
            .collect();

        new_nodes.append(&mut kid_nodes);
    }

    new_nodes
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample() {
        let lines = vec![
            "Valve AA has flow rate=0; tunnels lead to valves DD, II, BB".to_string(),
            "Valve BB has flow rate=13; tunnels lead to valves CC, AA".to_string(),
            "Valve CC has flow rate=2; tunnels lead to valves DD, BB".to_string(),
            "Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE".to_string(),
            "Valve EE has flow rate=3; tunnels lead to valves FF, DD".to_string(),
            "Valve FF has flow rate=0; tunnels lead to valves EE, GG".to_string(),
            "Valve GG has flow rate=0; tunnels lead to valves FF, HH".to_string(),
            "Valve HH has flow rate=22; tunnel leads to valve GG".to_string(),
            "Valve II has flow rate=0; tunnels lead to valves AA, JJ".to_string(),
            "Valve JJ has flow rate=21; tunnel leads to valve II".to_string(),
        ];

        assert_eq!(get_answer(lines), 1651)
    }
}
