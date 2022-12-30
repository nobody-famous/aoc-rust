use std::collections::{HashMap, HashSet};

pub const FILE_NAME: &str = "year2022/src/day16/puzzle.txt";

#[derive(Debug, Clone)]
pub struct Valve {
    name: String,
    flow: usize,
    kids: Vec<String>,
}

#[derive(Debug)]
pub struct Config {
    pub valves: HashMap<usize, Valve>,
    pub to_open: Vec<usize>,
    pub dist_map: HashMap<usize, HashMap<usize, usize>>,
    pub masks: HashMap<String, usize>,
}

pub fn walk(cfg: &Config, start: usize, max_time: usize) -> HashMap<usize, usize> {
    let seen: usize = 0;
    let mut flows: HashMap<usize, usize> = HashMap::new();

    walk_node(cfg, &mut flows, start, 0, 0, max_time, seen);

    flows
}

fn walk_node(
    cfg: &Config,
    flows: &mut HashMap<usize, usize>,
    node: usize,
    time: usize,
    flow: usize,
    max_time: usize,
    seen: usize,
) {
    for target in &cfg.to_open {
        if seen & target != 0 {
            continue;
        }

        let dists = match cfg.dist_map.get(&node) {
            Some(d) => d,
            None => todo!(),
        };
        let to_target = match dists.get(&target) {
            Some(dist) => dist,
            None => todo!(),
        };
        let target_valve = match cfg.valves.get(&target) {
            Some(valve) => valve,
            None => todo!(),
        };

        let new_time = time + to_target + 1;
        let rem_time = if new_time < max_time {
            max_time - new_time
        } else {
            0
        };
        let new_flow = flow + (target_valve.flow * rem_time);

        if new_time < max_time {
            let new_seen = seen | target;

            let old_flow = match flows.get(&new_seen) {
                Some(flow) => *flow,
                None => 0,
            };

            if new_flow > old_flow {
                let _ = flows.insert(new_seen, new_flow);
            }

            walk_node(cfg, flows, *target, new_time, new_flow, max_time, new_seen);
        }
    }
}

pub fn parse(lines: Vec<String>) -> Config {
    let valves: Vec<Valve> = lines.iter().map(|line| parse_valve(line)).collect();
    let to_open_names: Vec<String> = valves.iter().fold(vec![], |mut acc, valve| {
        if valve.flow > 0 {
            acc.push(valve.name.clone());
        }

        acc
    });
    let mut need_masks = to_open_names.clone();
    need_masks.push(String::from("AA"));

    let masks = assign_masks(&need_masks);
    let to_open: Vec<usize> = to_open_names
        .iter()
        .map(|name| get_mask(&masks, name))
        .collect();
    let valve_map: HashMap<String, Valve> = valves.iter().fold(HashMap::new(), |mut acc, valve| {
        acc.insert(valve.name.clone(), valve.clone());
        acc
    });
    let valve_map_masked: HashMap<usize, Valve> =
        valve_map
            .iter()
            .fold(HashMap::new(), |mut acc, (key, value)| {
                if masks.contains_key(key) {
                    acc.insert(get_mask(&masks, key), value.clone());
                }
                acc
            });
    let dist_map_raw = build_map(&valve_map, &to_open_names, "AA".to_string());
    let dist_map = dist_map_raw
        .iter()
        .fold(HashMap::new(), |mut acc, (key, value)| {
            if masks.contains_key(key) {
                let value_map = value.iter().fold(HashMap::new(), |mut acc, (key, value)| {
                    if masks.contains_key(key) {
                        acc.insert(get_mask(&masks, key), *value);
                    }
                    acc
                });
                acc.insert(get_mask(&masks, key), value_map);
            }
            acc
        });

    Config {
        valves: valve_map_masked,
        to_open,
        dist_map,
        masks,
    }
}

pub fn get_mask(masks: &HashMap<String, usize>, name: &String) -> usize {
    match masks.get(name) {
        Some(mask) => *mask,
        None => todo!(),
    }
}

fn assign_masks(names: &Vec<String>) -> HashMap<String, usize> {
    let mut masks: HashMap<String, usize> = HashMap::new();
    let mut shift = 0;

    for name in names {
        masks.insert(name.clone(), 1 << shift);
        shift += 1;
    }

    masks
}

fn build_map(
    valves: &HashMap<String, Valve>,
    to_open: &Vec<String>,
    start: String,
) -> HashMap<String, HashMap<String, usize>> {
    let mut dist_map: HashMap<String, HashMap<String, usize>> =
        to_open.iter().fold(HashMap::new(), |mut acc, name| {
            acc.insert(name.clone(), get_dists(valves, name));
            acc
        });

    dist_map.insert(start.clone(), get_dists(valves, &start));

    dist_map
}

fn get_dists(valves: &HashMap<String, Valve>, start: &String) -> HashMap<String, usize> {
    let mut dists: HashMap<String, usize> = HashMap::new();
    let start_valve = lookup_valve(&valves, start);
    let mut to_visit: Vec<String> = start_valve.kids.clone();
    let mut seen: HashSet<String> = HashSet::new();
    let mut dist: usize = 0;

    dists.insert(start.clone(), 0);
    seen.insert(start.clone());

    while to_visit.len() > 0 {
        let mut next_nodes: Vec<String> = vec![];
        dist += 1;

        for target in &to_visit {
            let valve = lookup_valve(valves, target);

            seen.insert(target.clone());
            dists.insert(target.clone(), dist);

            for kid in &valve.kids {
                if !seen.contains(kid) {
                    next_nodes.push(kid.clone());
                }
            }
        }

        to_visit = next_nodes;
    }

    dists
}

fn lookup_valve<'a>(valve_map: &'a HashMap<String, Valve>, name: &String) -> &'a Valve {
    match valve_map.get(name) {
        Some(valve) => valve,
        None => todo!(),
    }
}

fn parse_valve(line: &String) -> Valve {
    let parts: Vec<&str> = line.split(' ').collect();
    let name = parts[1];
    let flow = parse_flow(parts[4]);
    let kids: Vec<String> = parts[9..].iter().map(|kid| kid.replace(",", "")).collect();

    Valve {
        name: name.to_string(),
        flow,
        kids,
    }
}

fn parse_flow(input: &str) -> usize {
    let parts: Vec<String> = input.split('=').map(|p| p.replace(";", "")).collect();

    match parts[1].parse::<usize>() {
        Ok(rate) => rate,
        Err(_) => todo!(),
    }
}
