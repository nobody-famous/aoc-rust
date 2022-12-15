use crate::day9::utils::{do_move, State};

use super::utils::{parse_move, Move, FILE_NAME};

const CORRECT_ANSWER: usize = 6311;

pub fn solve() -> Result<(), String> {
    core::do_work(FILE_NAME, CORRECT_ANSWER, get_answer, |a, b| a == b)
}

fn get_answer(lines: Vec<String>) -> usize {
    let moves: Vec<Move> = lines.iter().map(|line| parse_move(line)).collect();
    let mut state = State::new();

    for to_move in moves.iter() {
        for _ in 0..to_move.dist {
            (state.head, state.tail) = do_move(&state.head, &state.tail, &to_move.dir);
            state.seen.insert(state.tail.clone());
        }
    }

    state.seen.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample() {
        let lines: Vec<String> = vec![
            "R 4".to_string(),
            "U 4".to_string(),
            "L 3".to_string(),
            "D 1".to_string(),
            "R 4".to_string(),
            "D 1".to_string(),
            "L 5".to_string(),
            "R 2".to_string(),
        ];

        assert_eq!(get_answer(lines), 13)
    }
}
