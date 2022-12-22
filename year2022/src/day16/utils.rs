use std::collections::HashMap;

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

pub fn build_map(
    to_mask: &HashMap<String, usize>,
    valves: &HashMap<usize, Valve>,
) -> HashMap<usize, HashMap<usize, usize>> {
    valves.iter().fold(HashMap::new(), |mut acc, (mask, _)| {
        acc.insert(*mask, get_dists(to_mask, &valves, *mask));
        acc
    })
}

pub fn get_mask(to_mask: &HashMap<String, usize>, name: &String) -> usize {
    match to_mask.get(name) {
        Some(mask) => *mask,
        None => todo!(),
    }
}

fn get_dists(
    to_mask: &HashMap<String, usize>,
    valves: &HashMap<usize, Valve>,
    start: usize,
) -> HashMap<usize, usize> {
    let mut dists: HashMap<usize, usize> = HashMap::new();
    let mut cur_dist: usize = 0;
    let mut to_visit = vec![start];
    let mut seen: usize = 0;

    dists.insert(start, 0);
    seen |= start;

    while to_visit.len() > 0 {
        cur_dist += 1;

        let mut next_nodes: Vec<usize> = vec![];
        while let Some(node) = to_visit.pop() {
            seen |= node;

            if let Some(valve) = valves.get(&node) {
                valve.kids.iter().for_each(|kid| {
                    let kid_mask = get_mask(to_mask, kid);
                    if seen & kid_mask == 0 {
                        let _ = dists.insert(kid_mask, cur_dist);
                        next_nodes.push(kid_mask);
                    }
                })
            }
        }

        to_visit = next_nodes;
    }

    dists
}

pub fn parse(lines: Vec<String>) -> (HashMap<String, usize>, HashMap<usize, Valve>) {
    let mut valves = HashMap::new();

    for line in lines {
        let (valve, kids) = parse_valve(line);
        valves.insert(valve, kids);
    }

    let keys: Vec<String> = valves.keys().map(|k| String::from(k)).collect();
    let (to_mask, _) = assign_masks(&keys);

    let out = valves
        .iter()
        .fold(HashMap::new(), |mut acc, (name, valve)| {
            match to_mask.get(name) {
                Some(mask) => acc.insert(*mask, valve.clone()),
                None => todo!(),
            };
            acc
        });

    (to_mask, out)
}

fn assign_masks(names: &Vec<String>) -> (HashMap<String, usize>, HashMap<usize, String>) {
    let mut shift = 0;
    let mut to = HashMap::new();
    let mut from = HashMap::new();

    names.iter().for_each(|name| {
        to.insert(name.clone(), 1 << shift);
        from.insert(1 << shift, name.clone());
        shift += 1;
    });

    (to, from)
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
