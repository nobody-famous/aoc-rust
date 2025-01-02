pub const FILE_NAME: &str = "year2022/src/day4/puzzle.txt";

#[derive(Debug, Clone)]
pub struct Range {
    pub start: u32,
    pub end: u32,
}

#[derive(Debug, Clone)]
pub struct Pair {
    pub left: Range,
    pub right: Range,
}

pub fn parse_pair(line: &str) -> Pair {
    let parts: Vec<Range> = line.split(',').map(parse_range).collect();

    Pair {
        left: parts[0].clone(),
        right: parts[1].clone(),
    }
}

pub fn parse_range(data: &str) -> Range {
    let parts: Vec<u32> = data.split('-').map(parse_int).collect();

    Range {
        start: parts[0],
        end: parts[1],
    }
}

pub fn parse_int(data: &str) -> u32 {
    data.parse::<u32>().unwrap_or(0)
}
