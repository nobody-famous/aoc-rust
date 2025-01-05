use core::AocResult;

pub fn solve(file_name: &str) -> AocResult<u32> {
    core::do_work(file_name, get_answer)
}

fn get_answer(lines: Vec<String>) -> AocResult<u32> {
    Ok(lines.iter().map(|line| round_score(line)).sum::<u32>())
}

fn round_score(line: &str) -> u32 {
    match line {
        "A X" => 3,
        "A Y" => 1 + 3,
        "A Z" => 2 + 6,
        "B X" => 1,
        "B Y" => 2 + 3,
        "B Z" => 3 + 6,
        "C X" => 2,
        "C Y" => 3 + 3,
        "C Z" => 1 + 6,
        _ => 0,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample() {
        let lines: Vec<String> =
            vec![String::from("A Y"), String::from("B X"), String::from("C Z")];

        assert_eq!(get_answer(lines).unwrap(), 12)
    }
}
