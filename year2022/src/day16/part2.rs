use std::collections::HashMap;

use super::utils::{build_map, get_mask, parse, Valve, FILE_NAME};

const CORRECT_ANSWER: usize = 1716;

pub fn solve() -> Result<(), String> {
    core::do_work(FILE_NAME, CORRECT_ANSWER, get_answer, |a, b| a == b)
}

struct Config {
    valves: HashMap<usize, Valve>,
    dist_map: HashMap<usize, HashMap<usize, usize>>,
    to_open: Vec<usize>,
    max_time: usize,
    highest: usize,
}

#[derive(Debug)]
struct Node {
    cur: usize,
    seen: usize,
    time: usize,
    flow: usize,
}

fn get_answer(lines: Vec<String>) -> usize {
    let (to_mask, valves) = parse(lines);
    let dist_map = build_map(&to_mask, &valves);
    let to_open = valves
        .iter()
        .filter(|(_, v)| v.rate > 0)
        .map(|(name, _)| name.clone())
        .collect::<Vec<usize>>();
    let mut cfg = Config {
        valves,
        dist_map,
        to_open,
        max_time: 26,
        highest: 0,
    };

    let dist = walk(
        &mut cfg,
        Node {
            cur: get_mask(&to_mask, &"AA".to_string()),
            seen: 0,
            time: 0,
            flow: 0,
        },
    );

    dist
}

fn get_flow(cfg: &Config, name: usize) -> usize {
    match cfg.valves.get(&name) {
        Some(valve) => valve.rate,
        None => todo!(),
    }
}

fn get_dist(dists_map: &HashMap<usize, HashMap<usize, usize>>, from: usize, to: usize) -> usize {
    match dists_map.get(&from) {
        Some(dists) => match dists.get(&to) {
            Some(dist) => *dist,
            None => {
                println!("No distance for {:?} -> {:?}", from, to);
                todo!()
            }
        },
        None => {
            println!("No distance for {:?} -> {:?}", from, to);
            todo!()
        }
    }
}

fn walk(cfg: &mut Config, node: Node) -> usize {
    let mut to_visit: Vec<Node> = get_to_visit(cfg, &node);

    while to_visit.len() > 0 {
        let mut next_nodes: Vec<Node> = vec![];

        for item in to_visit {
            if item.flow > cfg.highest {
                cfg.highest = item.flow;
            }

            let mut new_to_visit = get_to_visit(cfg, &item);
            next_nodes.append(&mut new_to_visit);
        }

        to_visit = next_nodes;
    }

    cfg.highest
}

fn to_node(cfg: &Config, node: &Node, target: usize) -> Node {
    let mut new_seen = node.seen;
    let dist = get_dist(&cfg.dist_map, node.cur, target);
    let flow = get_flow(cfg, target);
    let to_now = node.time + dist + 1;
    let rem_time = if to_now < cfg.max_time {
        cfg.max_time - to_now
    } else {
        0
    };
    let new_flow = node.flow + (flow * rem_time);

    new_seen |= target;

    Node {
        cur: target,
        seen: new_seen,
        time: node.time + dist + 1,
        flow: new_flow,
    }
}

fn get_to_visit(cfg: &Config, node: &Node) -> Vec<Node> {
    cfg.to_open
        .iter()
        .filter(|target| node.seen & *target == 0)
        .map(|target| to_node(cfg, node, *target))
        .filter(|node| node.time < cfg.max_time)
        .collect()
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

        assert_eq!(get_answer(lines), 1707)
    }
}
