use crate::day9::utils::{do_move, move_follower, State};

use super::utils::{parse_move, Move, FILE_NAME};

const CORRECT_ANSWER: usize = 2482;

pub fn solve() -> Result<(), String> {
    core::do_work(FILE_NAME, CORRECT_ANSWER, get_answer, |a, b| a == b)
}

fn get_answer(lines: Vec<String>) -> Result<usize, String> {
    let mut moves: Vec<Move> = vec![];
    let mut state = State::new(10);

    for line in lines {
        moves.push(parse_move(&line)?);
    }

    for to_move in moves.iter() {
        for _ in 0..to_move.dist {
            (state.knots[0], state.knots[1]) =
                do_move(&state.knots[0], &state.knots[1], &to_move.dir);

            for idx in 1..state.knots.len() {
                state.knots[idx] = move_follower(&state.knots[idx - 1], &state.knots[idx]);
            }

            state
                .seen
                .insert(state.knots[state.knots.len() - 1].clone());
        }
    }

    Ok(state.seen.len())
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

        assert_eq!(get_answer(lines), Ok(1))
    }

    #[test]
    fn sample_2() {
        let lines: Vec<String> = vec![
            "R 5".to_string(),
            "U 8".to_string(),
            "L 8".to_string(),
            "D 3".to_string(),
            "R 17".to_string(),
            "D 10".to_string(),
            "L 25".to_string(),
            "U 20".to_string(),
        ];

        assert_eq!(get_answer(lines), Ok(36))
    }
}
