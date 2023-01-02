use super::utils::{do_work, State, FILE_NAME};

const CORRECT_ANSWER: usize = 10096985;
const TOTAL_SPACE: usize = 70000000;
const TARGET_SPACE: usize = 30000000;

pub fn solve() -> Result<(), String> {
    core::do_work(FILE_NAME, CORRECT_ANSWER, get_answer, |a, b| a == b)
}

fn get_answer(lines: Vec<String>) -> Result<usize, String> {
    let state = do_work(lines, pop);
    let unused = TOTAL_SPACE - state.root;
    let answer = state
        .found
        .iter()
        .filter(|item| unused + *item >= TARGET_SPACE)
        .min();

    match answer {
        Some(n) => Ok(*n),
        None => Err(String::from("No min found")),
    }
}

fn pop(state: &mut State) {
    match state.stack.pop() {
        Some(n) => {
            state.found.push(n);

            match state.stack.pop() {
                Some(n1) => state.stack.push(n + n1),
                None => state.root = n,
            }
        }
        None => todo!(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample() {
        let lines: Vec<String> = vec![
            "$ cd /".to_string(),
            "$ ls".to_string(),
            "dir a".to_string(),
            "14848514 b.txt".to_string(),
            "8504156 c.dat".to_string(),
            "dir d".to_string(),
            "$ cd a".to_string(),
            "$ ls".to_string(),
            "dir e".to_string(),
            "29116 f".to_string(),
            "2557 g".to_string(),
            "62596 h.lst".to_string(),
            "$ cd e".to_string(),
            "$ ls".to_string(),
            "584 i".to_string(),
            "$ cd ..".to_string(),
            "$ cd ..".to_string(),
            "$ cd d".to_string(),
            "$ ls".to_string(),
            "4060174 j".to_string(),
            "8033020 d.log".to_string(),
            "5626152 d.ext".to_string(),
            "7214296 k".to_string(),
        ];

        assert_eq!(get_answer(lines), Ok(24933642));
    }
}
