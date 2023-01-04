pub const FILE_NAME: &str = "year2022/src/day19/puzzle.txt";

#[derive(Debug)]
struct Blueprint {}

pub fn parse(lines: Vec<String>) -> Result<(), String> {
    let groups = group_lines(&lines);
    let mut blueprints: Vec<()> = vec![];

    for group in &groups {
        blueprints.push(parse_blueprint(&group)?);
    }

    println!("GROUPS {:?}", groups);
    Err(String::from("parse Not Done Yet"))
}

fn parse_blueprint(group: &Vec<&String>) -> Result<(), String> {
    let mut num: usize = 0;

    for line in group {
        let parts: Vec<&str> = line.trim().split(' ').collect();

        println!("PARTS {:?}", parts);

        if parts[0] == "Blueprint" {
            num = match parts[1].replace(":", "").parse::<usize>() {
                Ok(n) => n,
                Err(e) => return Err(e.to_string()),
            }
        }
    }

    println!("{:?}", num);

    Err(String::from("parse_blueprint Not Done Yet"))
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
