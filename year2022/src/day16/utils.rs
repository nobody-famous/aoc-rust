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

pub fn walk(cfg: &Config, start: usize, max_time: usize) -> Result<HashMap<usize, usize>, String> {
    let seen: usize = 0;
    let mut flows: HashMap<usize, usize> = HashMap::new();

    walk_node(cfg, &mut flows, start, 0, 0, max_time, seen)?;

    Ok(flows)
}

fn walk_node(
    cfg: &Config,
    flows: &mut HashMap<usize, usize>,
    node: usize,
    time: usize,
    flow: usize,
    max_time: usize,
    seen: usize,
) -> Result<(), String> {
    for target in &cfg.to_open {
        if seen & target != 0 {
            continue;
        }

        let dists = get_cfg_dists_map(cfg, node)?;
        let to_target = get_dist_to_target(dists, *target)?;
        let target_valve = get_cfg_valve(cfg, *target)?;

        let new_time = time + to_target + 1;
        let rem_time = max_time.saturating_sub(new_time);
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

            walk_node(cfg, flows, *target, new_time, new_flow, max_time, new_seen)?;
        }
    }

    Ok(())
}

fn get_cfg_valve(cfg: &Config, target: usize) -> Result<&Valve, String> {
    match cfg.valves.get(&target) {
        Some(valve) => Ok(valve),
        None => Err(format!("Could not get valve for {:?}", target)),
    }
}

fn get_dist_to_target(dists: &HashMap<usize, usize>, target: usize) -> Result<usize, String> {
    match dists.get(&target) {
        Some(dist) => Ok(*dist),
        None => Err(format!("Could not find distance to {:?}", target)),
    }
}

fn get_cfg_dists_map(cfg: &Config, target: usize) -> Result<&HashMap<usize, usize>, String> {
    match cfg.dist_map.get(&target) {
        Some(d) => Ok(d),
        None => Err(format!("Could not find distances for {:?}", target)),
    }
}

pub fn parse(lines: Vec<String>) -> Result<Config, String> {
    let mut valves: Vec<Valve> = vec![];
    for line in &lines {
        valves.push(parse_valve(line)?);
    }

    let to_open_names: Vec<String> = valves.iter().fold(vec![], |mut acc, valve| {
        if valve.flow > 0 {
            acc.push(valve.name.clone());
        }

        acc
    });
    let mut need_masks = to_open_names.clone();
    need_masks.push(String::from("AA"));

    let masks = assign_masks(&need_masks);
    let mut to_open: Vec<usize> = vec![];
    for name in &to_open_names {
        to_open.push(get_mask(&masks, name)?);
    }

    let valve_map: HashMap<String, Valve> = valves.iter().fold(HashMap::new(), |mut acc, valve| {
        acc.insert(valve.name.clone(), valve.clone());
        acc
    });
    let mut valve_map_masked: HashMap<usize, Valve> = HashMap::new();
    for (key, value) in &valve_map {
        if masks.contains_key(key) {
            valve_map_masked.insert(get_mask(&masks, key)?, value.clone());
        }
    }

    let dist_map_raw = build_map(&valve_map, &to_open_names, "AA".to_string())?;
    let mut dist_map = HashMap::new();
    for (key, value) in dist_map_raw {
        if masks.contains_key(&key) {
            let mut value_map = HashMap::new();

            for (k, v) in value {
                if masks.contains_key(&k) {
                    value_map.insert(get_mask(&masks, &k)?, v);
                }
            }

            dist_map.insert(get_mask(&masks, &key)?, value_map);
        }
    }

    Ok(Config {
        valves: valve_map_masked,
        to_open,
        dist_map,
        masks,
    })
}

pub fn get_mask(masks: &HashMap<String, usize>, name: &String) -> Result<usize, String> {
    match masks.get(name) {
        Some(mask) => Ok(*mask),
        None => Err(format!("Could not get mask for {:?}", name)),
    }
}

fn assign_masks(names: &[String]) -> HashMap<String, usize> {
    let mut masks: HashMap<String, usize> = HashMap::new();

    for (shift, name) in names.iter().enumerate() {
        masks.insert(name.clone(), 1 << shift);
    }

    masks
}

fn build_map(
    valves: &HashMap<String, Valve>,
    to_open: &Vec<String>,
    start: String,
) -> Result<HashMap<String, HashMap<String, usize>>, String> {
    let mut dist_map: HashMap<String, HashMap<String, usize>> = HashMap::new();

    for name in to_open {
        dist_map.insert(name.clone(), get_dists(valves, name)?);
    }

    dist_map.insert(start.clone(), get_dists(valves, &start)?);

    Ok(dist_map)
}

fn get_dists(
    valves: &HashMap<String, Valve>,
    start: &String,
) -> Result<HashMap<String, usize>, String> {
    let mut dists: HashMap<String, usize> = HashMap::new();
    let start_valve = lookup_valve(valves, start)?;
    let mut to_visit: Vec<String> = start_valve.kids.clone();
    let mut seen: HashSet<String> = HashSet::new();
    let mut dist: usize = 0;

    dists.insert(start.clone(), 0);
    seen.insert(start.clone());

    while !to_visit.is_empty() {
        let mut next_nodes: Vec<String> = vec![];
        dist += 1;

        for target in &to_visit {
            let valve = lookup_valve(valves, target)?;

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

    Ok(dists)
}

fn lookup_valve<'a>(
    valve_map: &'a HashMap<String, Valve>,
    name: &String,
) -> Result<&'a Valve, String> {
    match valve_map.get(name) {
        Some(valve) => Ok(valve),
        None => Err(format!("Could not find value for {:?}", name)),
    }
}

fn parse_valve(line: &str) -> Result<Valve, String> {
    let parts: Vec<&str> = line.split(' ').collect();
    let name = parts[1];
    let flow = parse_flow(parts[4])?;
    let kids: Vec<String> = parts[9..].iter().map(|kid| kid.replace(",", "")).collect();

    Ok(Valve {
        name: name.to_string(),
        flow,
        kids,
    })
}

fn parse_flow(input: &str) -> Result<usize, String> {
    let parts: Vec<String> = input.split('=').map(|p| p.replace(";", "")).collect();

    match parts[1].parse::<usize>() {
        Ok(rate) => Ok(rate),
        Err(_) => Err(format!("Could not parse flow {}", parts[1])),
    }
}
