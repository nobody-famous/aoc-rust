use core::AocResult;

pub fn solve(file_name: &str) -> AocResult<usize> {
    core::do_work(file_name, get_answer)
}

fn get_answer(lines: Vec<String>) -> AocResult<usize> {
    Ok(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample() {
        let lines = vec![
            String::from("498,4 -> 498,6 -> 496,6"),
            String::from("503,4 -> 502,4 -> 502,9 -> 494,9"),
        ];

        assert_eq!(get_answer(lines).unwrap(), 24)
    }
}
