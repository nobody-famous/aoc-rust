use aoc::ProblemResult;

pub const FILE_NAME: &str = "year2022/src/day7/puzzle.txt";

#[derive(Debug)]
pub struct State {
    pub stack: Vec<usize>,
    pub found: Vec<usize>,
    pub root: usize,
}

impl State {
    pub fn new() -> Self {
        State {
            stack: vec![0],
            found: vec![],
            root: 0,
        }
    }
}

pub fn do_work(lines: Vec<String>, pop: fn(&mut State) -> ProblemResult<()>) -> ProblemResult<State> {
    let mut state = State::new();

    for line in lines {
        if line.starts_with('$') {
            cmd(&mut state, line, pop)?;
        } else if !line.starts_with("dir") {
            handle_file(&mut state, line)?;
        }
    }

    while !state.stack.is_empty() {
        pop(&mut state)?;
    }

    Ok(state)
}

fn handle_file(state: &mut State, line: String) -> Result<(), String> {
    let parts: Vec<&str> = line.split(' ').collect();

    match parts.get(0) {
        Some(s) => match s.parse::<usize>() {
            Ok(n) => match state.stack.pop() {
                Some(item) => {
                    state.stack.push(item + n);
                    Ok(())
                }
                None => Err(String::from("Stack is empty")),
            },
            Err(_) => Err(format!("Failed to parse {:?}", s)),
        },
        None => Err(String::from("No first entry in parts")),
    }
}

fn cmd(state: &mut State, line: String, pop: fn(&mut State) -> ProblemResult<()>) -> ProblemResult<()> {
    let parts: Vec<&str> = line.split(' ').collect();

    match (parts.get(1), parts.get(2)) {
        (Some(s1), Some(s2)) if *s1 == "cd" && *s2 != ".." => {
            state.stack.push(0);
            Ok(())
        }
        (Some(s1), Some(s2)) if *s1 == "cd" && *s2 == ".." => pop(state),
        _ => Ok(()),
    }
}
