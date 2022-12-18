use std::collections::{BinaryHeap, HashSet};

use crate::day16::utils::State;

use super::utils::{build_map, parse, FILE_NAME};

const CORRECT_ANSWER: usize = 0;

pub fn solve() -> Result<(), String> {
    core::do_work(FILE_NAME, CORRECT_ANSWER, get_answer, |a, b| a == b)
}

#[derive(Debug, PartialEq, Eq)]
struct ToVisit {
    pub name: String,
    pub seen: HashSet<String>,
    pub dist: usize,
}

impl ToVisit {
    pub fn new(name: String, seen: HashSet<String>, dist: usize) -> Self {
        ToVisit { name, seen, dist }
    }
}

impl Ord for ToVisit {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.dist.cmp(&self.dist)
    }
}

impl PartialOrd for ToVisit {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        other.dist.partial_cmp(&self.dist)
    }
}

fn get_answer(lines: Vec<String>) -> usize {
    let valves = parse(lines);
    let dist_map = build_map(&valves);
    let state = State::new(valves, dist_map);
    let mut to_visit: BinaryHeap<ToVisit> = BinaryHeap::new();

    to_visit.push(ToVisit::new(String::from("AA"), HashSet::new(), 0));

    while let Some(node) = to_visit.pop() {
        println!("NODE {:?}", node);
        if let (Some(aa_map), Some(aa_valve)) =
            (state.dist_map.get(&node.name), state.valves.get(&node.name))
        {
            println!("  {:?} {:?}", node.name, aa_valve);

            state.to_open.iter().for_each(|name| {
                if let (Some(dist), Some(valve)) = (aa_map.get(name), state.valves.get(name)) {
                    println!(
                        "    {:?}: {:?} * {:?} = {:?}",
                        name,
                        dist + 1,
                        valve.rate,
                        (dist + 1) * valve.rate
                    );
                }
            });
        }
    }

    0
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
