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
    pub opened: HashSet<String>,
    pub flow: usize,
    pub minutes: usize,
}

impl State {
    pub fn new(valves: HashMap<String, Valve>) -> Self {
        State {
            valves,
            opened: HashSet::new(),
            flow: 0,
            minutes: 0,
        }
    }
}

#[derive(Debug, Clone)]
pub struct ToVisit {
    pub state: State,
    pub name: String,
}

impl ToVisit {
    pub fn new(state: State, name: String) -> Self {
        ToVisit { state, name }
    }
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
