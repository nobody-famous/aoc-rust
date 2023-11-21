const CORRECT_ANSWER: &str = "TBVFVDZPN";

use aoc::ProblemResult;

use crate::{day5::utils::parse, day5::utils::FILE_NAME};

use super::utils::State;

pub fn solve() -> ProblemResult<()> {
    aoc::do_work(
        FILE_NAME,
        String::from(CORRECT_ANSWER),
        get_answer,
        |a, b| a.eq(b),
    )
}

fn get_answer(lines: Vec<String>) -> ProblemResult<String> {
    let mut state = parse(lines);

    for idx in 0..state.moves.len() {
        do_move(&mut state, idx)?;
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

fn do_move(state: &mut State, idx: usize) -> Result<(), String> {
    let to_move = &state.moves[idx];

    for _ in 0..to_move.count {
        match state.stacks[to_move.from - 1].pop() {
            Some(item) => state.stacks[to_move.to - 1].push(item),
            None => return Err(String::from("Stack is empty")),
        }
    }

    Ok(())
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

        assert_eq!(get_answer(lines).unwrap(), String::from("CMZ"))
    }
}
