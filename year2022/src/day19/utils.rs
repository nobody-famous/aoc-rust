use std::collections::HashMap;

pub const FILE_NAME: &str = "year2022/src/day19/puzzle.txt";

#[derive(Debug)]
pub struct Blueprint {
    pub num: usize,
    pub robots: HashMap<String, HashMap<String, usize>>,
}

#[derive(Debug)]
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
            ore: 1,
            clay: 0,
            obsidian: 0,
            ore_robots: 0,
            clay_robots: 0,
            obsidian_robots: 0,
        }
    }
}

pub fn parse(lines: Vec<String>) -> Result<Vec<Blueprint>, String> {
    let groups = group_lines(&lines);
    let mut blueprints: Vec<Blueprint> = vec![];

    for group in &groups {
        blueprints.push(parse_blueprint(&group)?);
    }

    Ok(blueprints)
}

fn parse_blueprint(group: &Vec<&String>) -> Result<Blueprint, String> {
    let mut num: usize = 0;
    let mut robots: HashMap<String, HashMap<String, usize>> = HashMap::new();

    for line in group {
        let parts: Vec<&str> = line.trim().split(' ').collect();

        match parts[0] {
            "Blueprint" => {
                num = match parts[1].replace(":", "").parse::<usize>() {
                    Ok(n) => n,
                    Err(e) => return Err(e.to_string()),
                }
            }
            "Each" => {
                let robot_type = parts[1];
                let costs = parse_costs(&parts[4..])?;

                robots.insert(robot_type.to_string(), costs);
            }
            _ => return Err(format!("Invalid start of line: {:?}", parts[0])),
        }
    }

    Ok(Blueprint { num, robots })
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

fn group_lines<'a>(lines: &'a Vec<String>) -> Vec<Vec<&'a String>> {
    let mut groups = vec![];
    let mut cur_group = vec![];

    for line in lines {
        if line.len() == 0 {
            groups.push(cur_group);
            cur_group = vec![];
        } else {
            cur_group.push(line);
        }
    }

    if cur_group.len() > 0 {
        groups.push(cur_group);
    }

    groups
}
