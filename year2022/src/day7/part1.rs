use core::AocResult;

use super::utils::{do_work, State, FILE_NAME};

const CORRECT_ANSWER: usize = 1501149;
const TARGET: usize = 100000;

pub fn solve() -> AocResult<()> {
    core::do_work(FILE_NAME, CORRECT_ANSWER, get_answer, |a, b| a == b)
}

fn get_answer(lines: Vec<String>) -> AocResult<usize> {
    let state = do_work(lines, pop)?;
    Ok(state.found.iter().sum::<usize>())
}

fn pop(state: &mut State) -> AocResult<()> {
    match state.stack.pop() {
        Some(n) => {
            if n <= TARGET {
                state.found.push(n);
            }

            if let Some(n1) = state.stack.pop() {
                state.stack.push(n + n1)
            };

            Ok(())
        }
        None => Err("Stack is empty".into()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample() {
        let lines: Vec<String> = vec![
            String::from("$ cd /"),
            String::from("$ ls"),
            String::from("dir a"),
            String::from("14848514 b.txt"),
            String::from("8504156 c.dat"),
            String::from("dir d"),
            String::from("$ cd a"),
            String::from("$ ls"),
            String::from("dir e"),
            String::from("29116 f"),
            String::from("2557 g"),
            String::from("62596 h.lst"),
            String::from("$ cd e"),
            String::from("$ ls"),
            String::from("584 i"),
            String::from("$ cd .."),
            String::from("$ cd .."),
            String::from("$ cd d"),
            String::from("$ ls"),
            String::from("4060174 j"),
            String::from("8033020 d.log"),
            String::from("5626152 d.ext"),
            String::from("7214296 k"),
        ];

        assert_eq!(get_answer(lines).unwrap(), 95437);
    }
}
