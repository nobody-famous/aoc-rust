use core::AocResult;

use super::utils::{do_move, parse_move, Move, State};

pub fn solve(file_name: &str) -> AocResult<usize> {
    core::do_work(file_name, get_answer)
}

fn get_answer(lines: Vec<String>) -> AocResult<usize> {
    let mut moves: Vec<Move> = vec![];
    let mut state = State::new(2);

    for line in lines {
        moves.push(parse_move(&line)?);
    }

    for to_move in moves.iter() {
        for _ in 0..to_move.dist {
            (state.knots[0], state.knots[1]) =
                do_move(&state.knots[0], &state.knots[1], &to_move.dir);
            state.seen.insert(state.knots[1]);
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
            String::from("R 4"),
            String::from("U 4"),
            String::from("L 3"),
            String::from("D 1"),
            String::from("R 4"),
            String::from("D 1"),
            String::from("L 5"),
            String::from("R 2"),
        ];

        assert_eq!(get_answer(lines).unwrap(), 13)
    }
}
