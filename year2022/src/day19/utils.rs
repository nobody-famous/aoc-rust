use std::collections::HashMap;

pub const FILE_NAME: &str = "year2022/src/day19/puzzle.txt";

#[derive(Debug)]
pub struct Blueprint {
    pub num: usize,
    pub rules: HashMap<String, HashMap<String, usize>>,
}

#[derive(Debug, Clone)]
pub struct State {
    pub ore: usize,
    pub clay: usize,
    pub obsidian: usize,
    pub ore_robots: usize,
    pub clay_robots: usize,
    pub obsidian_robots: usize,
}

impl State {
    pub fn new() -> Self {
        State {
            ore: 0,
            clay: 0,
            obsidian: 0,
            ore_robots: 1,
            clay_robots: 0,
            obsidian_robots: 0,
        }
    }
}

pub fn parse(lines: Vec<String>) -> Result<Vec<Blueprint>, String> {
    lines
        .iter()
        .map(|line| {
            let parts = line.split(':').collect::<Vec<&str>>();
            let num = parse_num(parts[0])?;
            let rules = parse_rules(parts[1])?;

            Ok(Blueprint { num, rules })
        })
        .collect()
}

fn parse_num(input: &str) -> Result<usize, String> {
    let parts: Vec<&str> = input.trim().split(' ').collect();

    match parts[1].parse::<usize>() {
        Ok(n) => Ok(n),
        Err(e) => return Err(e.to_string()),
    }
}

fn parse_rules(input: &str) -> Result<HashMap<String, HashMap<String, usize>>, String> {
    let mut rules: HashMap<String, HashMap<String, usize>> = HashMap::new();
    let parts = input.trim().split('.').collect::<Vec<&str>>();

    for part in parts {
        if part.len() == 0 {
            continue;
        }

        let pieces: Vec<&str> = part.trim().split(' ').collect();
        let name = pieces[1];
        let costs = parse_costs(&pieces[4..])?;

        rules.insert(name.to_string(), costs);
    }

    Ok(rules)
}

fn parse_costs(input: &[&str]) -> Result<HashMap<String, usize>, String> {
    let mut ndx = 0;
    let mut costs: HashMap<String, usize> = HashMap::new();

    while ndx < input.len() {
        let cost = match input[ndx].parse::<usize>() {
            Ok(c) => c,
            Err(e) => return Err(e.to_string()),
        };
        let item = input[ndx + 1];

        costs.insert(item.to_string(), cost);

        ndx += 2;

        if ndx < input.len() && input[ndx] == "and" {
            ndx += 1;
        }
    }

    Ok(costs)
}
