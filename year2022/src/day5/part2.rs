use core::AocResult;

use super::utils::{parse, State};

pub fn solve(file_name: &str) -> AocResult<String> {
    core::do_work(file_name, get_answer)
}

fn get_answer(lines: Vec<String>) -> AocResult<String> {
    let mut state = parse(lines);

    for idx in 0..state.moves.len() {
        do_move(&mut state, idx);
    }

    Ok(state
        .stacks
        .iter()
        .map(|stack| match stack.last() {
            Some(ch) => *ch,
            None => ' ',
        })
        .collect())
}

fn do_move(state: &mut State, idx: usize) {
    let to_do = &state.moves[idx];
    let size = state.stacks[to_do.from - 1].len();
    let mut to_move: Vec<char> = state.stacks[to_do.from - 1].drain(size - to_do.count..).collect();

    state.stacks[to_do.to - 1].append(&mut to_move);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample() {
        let lines: Vec<String> = vec![
            String::from("    [D]    "),
            String::from("[N] [C]    "),
            String::from("[Z] [M] [P]"),
            String::from(" 1   2   3 "),
            String::from(""),
            String::from("move 1 from 2 to 1"),
            String::from("move 3 from 1 to 3"),
            String::from("move 2 from 2 to 1"),
            String::from("move 1 from 1 to 2"),
        ];

        assert_eq!(get_answer(lines).unwrap(), String::from("MCD"))
    }
}
