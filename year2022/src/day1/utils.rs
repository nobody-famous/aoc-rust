pub const FILE_NAME: &str = "year2022/src/day1/puzzle.txt";

pub fn parse(lines: Vec<String>) -> Vec<u32> {
    let groups = group_lines(&lines);
    let values = parse_ints(&groups);

    sum_ints(&values)
}

fn sum_ints(values: &Vec<Vec<u32>>) -> Vec<u32> {
    values.iter().map(|group| group.iter().sum()).collect()
}

fn parse_ints(lines: &Vec<Vec<String>>) -> Vec<Vec<u32>> {
    lines
        .iter()
        .map(|group| {
            group
                .iter()
                .map(|item| match item.parse() {
                    Ok(num) => num,
                    Err(_) => 0,
                })
                .collect()
        })
        .collect()
}

fn group_lines(lines: &Vec<String>) -> Vec<Vec<String>> {
    lines.iter().fold(vec![vec![]], |mut acc, line| {
        if line.len() == 0 {
            acc.push(vec![]);
        } else if let Some(mut group) = acc.pop() {
            group.push(line.clone());
            acc.push(group);
        }

        acc
    })
}
