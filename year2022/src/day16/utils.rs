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

#[derive(Debug)]
pub struct Node {
    pub cur: usize,
    pub seen: usize,
    pub time: usize,
    pub flow: usize,
}

#[derive(Debug)]
pub struct Config {
    pub masks: HashMap<String, usize>,
    pub valves: HashMap<usize, Valve>,
    pub dist_map: HashMap<usize, HashMap<usize, usize>>,
    pub to_open: Vec<usize>,
    pub max_time: usize,
}

pub fn create_config(lines: Vec<String>, max_time: usize) -> Config {
    let valves = parse(lines);
    let to_open = valves
        .iter()
        .filter(|(_, v)| v.rate > 0)
        .map(|(name, _)| name.clone())
        .collect::<Vec<String>>();

    let mut tmp = to_open.clone();
    tmp.push(String::from("AA"));

    let masks = assign_masks(&tmp);
    let dist_map = build_map(&masks, &tmp, &valves);
    let new_to_open = to_open.iter().map(|name| get_mask(&masks, name)).collect();
    let new_valves = valves
        .iter()
        .filter(|(key, _)| masks.contains_key(*key))
        .fold(HashMap::new(), |mut acc, (key, value)| {
            acc.insert(get_mask(&masks, key), value.clone());
            acc
        });

    Config {
        masks,
        valves: new_valves,
        dist_map,
        to_open: new_to_open,
        max_time,
    }
}

pub fn walk(cfg: &mut Config, node: Node) -> HashMap<usize, usize> {
    let mut dists: HashMap<usize, usize> = HashMap::new();
    let mut to_visit: Vec<Node> = get_to_visit(cfg, &node);

    while to_visit.len() > 0 {
        let mut next_nodes: Vec<Node> = vec![];

        for item in to_visit {
            update_dist(&mut dists, item.seen, item.flow);

            let mut new_to_visit = get_to_visit(cfg, &item);
            next_nodes.append(&mut new_to_visit);
        }

        to_visit = next_nodes;
    }

    dists
}

pub fn get_mask(to_mask: &HashMap<String, usize>, name: &String) -> usize {
    match to_mask.get(name) {
        Some(mask) => *mask,
        None => todo!(),
    }
}

fn update_dist(dists: &mut HashMap<usize, usize>, key: usize, value: usize) {
    let old_value: usize = *dists.get(&key).unwrap_or(&0);

    if value > old_value {
        dists.insert(key, value);
    }
}

fn build_map(
    masks: &HashMap<String, usize>,
    to_open: &Vec<String>,
    valves: &HashMap<String, Valve>,
) -> HashMap<usize, HashMap<usize, usize>> {
    to_open.iter().fold(HashMap::new(), |mut acc, name| {
        let mask = get_mask(masks, name);
        acc.insert(mask, get_dists(masks, &valves, name));
        acc
    })
}

fn get_dists(
    masks: &HashMap<String, usize>,
    valves: &HashMap<String, Valve>,
    start: &String,
) -> HashMap<usize, usize> {
    let mut dists: HashMap<usize, usize> = HashMap::new();
    let mut cur_dist: usize = 0;
    let mut to_visit = vec![start];
    let mut seen: HashSet<&String> = HashSet::new();
    let start_mask = get_mask(masks, start);

    dists.insert(start_mask, 0);
    seen.insert(start);

    while to_visit.len() > 0 {
        cur_dist += 1;

        let mut next_nodes: Vec<&String> = vec![];
        while let Some(node) = to_visit.pop() {
            seen.insert(node);

            if let Some(valve) = valves.get(node) {
                valve.kids.iter().for_each(|kid| {
                    if !seen.contains(kid) {
                        if let Some(kid_mask) = masks.get(kid) {
                            let _ = dists.insert(*kid_mask, cur_dist);
                        }
                        next_nodes.push(kid);
                    }
                })
            }
        }

        to_visit = next_nodes;
    }

    dists
}

fn parse(lines: Vec<String>) -> HashMap<String, Valve> {
    let mut valves = HashMap::new();

    for line in lines {
        let (valve, kids) = parse_valve(line);
        valves.insert(valve, kids);
    }

    valves
}

fn assign_masks(names: &Vec<String>) -> HashMap<String, usize> {
    let mut shift = 0;
    let mut to = HashMap::new();

    names.iter().for_each(|name| {
        to.insert(name.clone(), 1 << shift);
        shift += 1;
    });

    to
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

fn to_node(cfg: &Config, node: &Node, target: usize) -> Node {
    let mut new_seen = node.seen;
    let dist = get_dist(&cfg.dist_map, node.cur, target);
    let flow = get_flow(cfg, target);
    let tmp = node.time + dist + 1;
    let rem_time = if cfg.max_time > tmp {
        cfg.max_time - tmp
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
