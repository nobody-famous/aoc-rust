use std::collections::{HashMap, HashSet};

pub const FILE_NAME: &str = "year2022/src/day16/puzzle.txt";

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
pub struct Valve {
    pub rate: usize,
    pub kids: Vec<String>,
}

impl Valve {
    pub fn new(rate: usize, kids: Vec<String>) -> Self {
        Valve { rate, kids }
    }
}

#[derive(Debug, Clone)]
pub struct State {
    pub valves: HashMap<String, Valve>,
    pub to_open: Vec<String>,
    pub opened: HashSet<String>,
    pub flow: usize,
    pub minutes: usize,
}

impl State {
    pub fn new(valves: HashMap<String, Valve>) -> Self {
        let to_open = valves
            .iter()
            .filter(|(_, v)| v.rate > 0)
            .map(|(name, _)| name.clone())
            .collect::<Vec<String>>();

        State {
            valves,
            to_open,
            opened: HashSet::new(),
            flow: 0,
            minutes: 0,
        }
    }
}

pub fn build_map(valves: &HashMap<String, Valve>) -> HashMap<String, HashMap<String, usize>> {
    valves.iter().fold(HashMap::new(), |mut acc, (name, _)| {
        acc.insert(name.clone(), get_dists(&valves, name));
        acc
    })
}

fn get_dists(valves: &HashMap<String, Valve>, start: &String) -> HashMap<String, usize> {
    let mut dists: HashMap<String, usize> = HashMap::new();
    let mut cur_dist: usize = 0;
    let mut to_visit: Vec<String> = vec![start.clone()];
    let mut seen: HashSet<String> = HashSet::new();

    dists.insert(start.clone(), 0);
    seen.insert(start.clone());

    while to_visit.len() > 0 {
        cur_dist += 1;

        let mut next_nodes: Vec<String> = vec![];
        while let Some(node) = to_visit.pop() {
            seen.insert(node.clone());

            if let Some(valve) = valves.get(&node) {
                valve.kids.iter().for_each(|kid| {
                    if !seen.contains(kid) {
                        let _ = dists.insert(kid.clone(), cur_dist);
                        next_nodes.push(kid.clone());
                    }
                })
            }
        }

        to_visit = next_nodes;
    }

    dists
}

pub fn parse(lines: Vec<String>) -> HashMap<String, Valve> {
    let mut valves = HashMap::new();

    for line in lines {
        let (valve, kids) = parse_valve(line);
        valves.insert(valve, kids);
    }

    valves
}

fn parse_valve(line: String) -> (String, Valve) {
    let pieces: Vec<&str> = line.split(' ').collect();
    let name = pieces[1];
    let rate = parse_rate(pieces[4]);
    let (_, kids) = pieces.split_at(9);

    (
        name.to_string(),
        Valve::new(
            rate,
            kids.iter()
                .map(|s| s.trim_end_matches(',').to_string())
                .collect(),
        ),
    )
}

fn parse_rate(data: &str) -> usize {
    let pieces: Vec<&str> = data.split('=').collect();

    match pieces[1].trim_end_matches(';').parse::<usize>() {
        Ok(n) => n,
        Err(_) => todo!(),
    }
}
