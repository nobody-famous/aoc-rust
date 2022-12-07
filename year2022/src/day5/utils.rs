pub const FILE_NAME: &str = "year2022/src/day5/puzzle.txt";

pub enum ParseState {
    Stacks,
    IDs,
    Moves,
}

#[derive(Debug)]
pub struct Move {
    pub count: usize,
    pub from: usize,
    pub to: usize,
}

#[derive(Debug)]
pub struct Dock {
    pub stacks: Vec<Vec<char>>,
    pub moves: Vec<Move>,
}

impl Dock {
    pub fn new(size: usize) -> Self {
        Dock {
            stacks: vec![vec![]; size],
            moves: vec![],
        }
    }

    pub fn add(self: &mut Self, idx: usize, ch: char) {
        self.stacks[idx].push(ch)
    }
}

pub fn parse(lines: Vec<String>) -> Dock {
    let (stacks, ids, moves) = split_input(&lines);
    let size = get_dock_size(ids);
    let mut dock = Dock::new(size);

    stacks.iter().rev().for_each(|row| {
        for i in 0..dock.stacks.len() {
            let idx = i * 4 + 1;
            if idx < row.len() {
                if let Some(ch) = row.chars().nth(idx) {
                    if ch != ' ' {
                        dock.add(i, ch);
                    }
                }
            }
        }
    });

    dock.moves = moves.iter().map(|line| parse_move(line)).collect();

    dock
}

fn parse_move(line: &str) -> Move {
    let pieces: Vec<usize> = line
        .split(' ')
        .skip(1)
        .step_by(2)
        .map(|item| match item.parse() {
            Ok(n) => n,
            Err(_) => 0,
        })
        .collect();

    Move {
        count: pieces[0],
        from: pieces[1],
        to: pieces[2],
    }
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
