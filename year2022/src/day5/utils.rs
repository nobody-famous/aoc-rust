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
pub struct State {
    pub stacks: Vec<Vec<char>>,
    pub moves: Vec<Move>,
}

impl State {
    pub fn new(size: usize) -> Self {
        State {
            stacks: vec![vec![]; size],
            moves: vec![],
        }
    }

    pub fn add(&mut self, idx: usize, ch: char) {
        self.stacks[idx].push(ch)
    }
}

pub fn parse(lines: Vec<String>) -> State {
    let (stacks, ids, moves) = split_input(&lines);
    let size = get_state_size(ids);
    let mut state = State::new(size);

    stacks.iter().rev().for_each(|row| {
        for i in 0..state.stacks.len() {
            let idx = i * 4 + 1;
            if idx < row.len() {
                if let Some(ch) = row.chars().nth(idx) {
                    if ch != ' ' {
                        state.add(i, ch);
                    }
                }
            }
        }
    });

    state.moves = moves.iter().map(parse_move).collect();

    state
}

fn parse_move(line: &&String) -> Move {
    let pieces: Vec<usize> = line
        .split(' ')
        .skip(1)
        .step_by(2)
        .map(|item| item.parse().unwrap_or(0))
        .collect();

    Move {
        count: pieces[0],
        from: pieces[1],
        to: pieces[2],
    }
}

fn get_state_size(ids: &str) -> usize {
    if let Some(s) = ids.split_whitespace().last() {
        if let Ok(n) = s.parse::<usize>() {
            return n;
        }
    }

    0
}

fn split_input(lines: &[String]) -> (Vec<&String>, &str, Vec<&String>) {
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
