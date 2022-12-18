use std::collections::{BinaryHeap, HashMap, HashSet};

use crate::day16::utils::State;

use super::utils::{build_map, parse, FILE_NAME};

const CORRECT_ANSWER: usize = 0;

pub fn solve() -> Result<(), String> {
    core::do_work(FILE_NAME, CORRECT_ANSWER, get_answer, |a, b| a == b)
}

#[derive(Debug, PartialEq, Eq)]
struct ToVisit {
    pub name: String,
    pub remaining: HashSet<String>,
    pub steps: usize,
    pub dist: usize,
    pub path: Vec<String>,
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

fn print_dists(dists: &HashMap<String, HashMap<String, usize>>) {
    let mut dist_vec: Vec<(&String, &HashMap<String, usize>)> = dists.iter().map(|e| e).collect();

    dist_vec.sort_by(|(n1, _), (n2, _)| n1.cmp(n2));

    dist_vec.iter().for_each(|(name, dist)| {
        let mut v: Vec<(&String, &usize)> = dist.iter().map(|e| e).collect();

        v.sort_by(|(n1, _), (n2, _)| n1.cmp(n2));

        print!("{:?}", name);
        v.iter().for_each(|(_, s)| print!(" {:?}", s));
        println!();
    });
}

fn get_answer(lines: Vec<String>) -> usize {
    let valves = parse(lines);
    let dist_map = build_map(&valves);
    let max_flow = valves.iter().fold(0, |acc, (_, valve)| acc + valve.rate) * 30;
    let state = State::new(valves, dist_map);
    let mut to_visit: BinaryHeap<ToVisit> = BinaryHeap::new();
    let mut answer: Option<usize> = None;

    print_dists(&state.dist_map);

    to_visit.push(ToVisit {
        name: String::from("AA"),
        remaining: state.to_open.iter().fold(HashSet::new(), |mut acc, v| {
            acc.insert(v.clone());
            acc
        }),
        dist: 0,
        steps: 0,
        path: vec![],
    });

    while answer == None {
        if let Some(node) = to_visit.pop() {
            if node.remaining.len() == 0 {
                answer = Some(max_flow - node.dist);
            }

            if let Some(dists) = state.dist_map.get(&node.name) {
                node.remaining.iter().for_each(|name| {
                    if let (Some(dist), Some(valve)) = (dists.get(name), state.valves.get(name)) {
                        let new_steps = node.steps + dist + 1;
                        let new_dist = (new_steps * valve.rate) + node.dist;
                        let mut new_rem = node.remaining.clone();
                        let mut new_path = node.path.clone();

                        new_rem.remove(name);
                        new_path.push(node.name.clone());

                        to_visit.push(ToVisit {
                            name: name.clone(),
                            remaining: new_rem,
                            steps: new_steps,
                            dist: new_dist,
                            path: new_path,
                        })
                    }
                });
            } else {
                assert!(false);
            }
        } else {
            assert!(false);
        }
    }

    match answer {
        Some(n) => n,
        None => todo!(),
    }
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

        // 0 0 A
        // 0 0 A,D

        // A 0
        // B 13
        // C 2
        // D 20
        // E 3
        // F 0
        // G 0
        // H 22
        // I 0
        // J 21

        //   A B C D E F G H I J
        // A 0 1 2 1 2 3 4 5 1 2
        // B 1 0 1 2 3 4 5 6 2 3
        // C 2 1 0 1 2 3 4 5 3 4
        // D 1 2 1 0 1 2 3 4 2 3
        // E 2 3 2 1 0 1 2 3 3 4
        // F 3 4 3 2 1 0 1 2 4 5
        // G 4 5 4 3 2 1 0 1 5 6
        // H 5 6 5 4 3 2 1 0 6 7
        // I 1 2 3 2 3 4 5 6 0 1
        // J 2 3 4 3 4 5 6 7 1 0

        assert_eq!(get_answer(lines), 1651)
    }
}
