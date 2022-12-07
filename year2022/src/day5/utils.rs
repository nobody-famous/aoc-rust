pub const FILE_NAME: &str = "year2022/src/day5/puzzle.txt";

pub enum ParseState {
    Stacks,
    IDs,
    Moves,
}

pub struct Dock {
    pub stacks: Vec<Vec<char>>,
}

impl Dock {
    pub fn new(size: usize) -> Self {
        Dock {
            stacks: vec![vec![]; size],
        }
    }
}

pub fn parse(lines: Vec<String>) -> Dock {
    let (stacks, ids, moves) = split_input(&lines);
    let size = get_dock_size(ids);
    let dock = Dock::new(size);

    dock
}

fn get_dock_size(ids: &str) -> usize {
    let parts: Option<&str> = ids.split_whitespace().last();

    match parts {
        Some(s) => match s.parse::<usize>() {
            Ok(n) => n,
            _ => 0,
        },
        _ => 0,
    }
}

fn split_input(lines: &Vec<String>) -> (Vec<&String>, &str, Vec<&String>) {
    let mut stacks: Vec<&String> = vec![];
    let mut ids: &str = "";
    let mut moves: Vec<&String> = vec![];
    let mut state: ParseState = ParseState::Stacks;

    lines.iter().for_each(|line| match state {
        ParseState::Stacks => {
            if line.contains('[') {
                stacks.push(line)
            } else {
                ids = line;
                state = ParseState::IDs
            }
        }
        ParseState::IDs => state = ParseState::Moves,
        ParseState::Moves => moves.push(line),
    });

    (stacks, ids, moves)
}
