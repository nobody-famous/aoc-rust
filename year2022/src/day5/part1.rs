const CORRECT_ANSWER: &str = "TBVFVDZPN";

use crate::{day5::utils::parse, day5::utils::FILE_NAME};

use super::utils::State;

pub fn solve() -> Result<(), String> {
    core::do_work(
        FILE_NAME,
        String::from(CORRECT_ANSWER),
        get_answer,
        |a, b| a.eq(b),
    )
}

fn get_answer(lines: Vec<String>) -> Result<String, String> {
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
            "    [D]    ".to_string(),
            "[N] [C]    ".to_string(),
            "[Z] [M] [P]".to_string(),
            " 1   2   3 ".to_string(),
            "".to_string(),
            "move 1 from 2 to 1".to_string(),
            "move 3 from 1 to 3".to_string(),
            "move 2 from 2 to 1".to_string(),
            "move 1 from 1 to 2".to_string(),
        ];

        assert_eq!(get_answer(lines), Ok(String::from("CMZ")))
    }
}
