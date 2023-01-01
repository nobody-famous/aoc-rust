use super::utils::{parse, round, FILE_NAME};

const CORRECT_ANSWER: usize = 0;

pub fn solve() -> Result<(), String> {
    core::do_work(FILE_NAME, CORRECT_ANSWER, get_answer, |a, b| a == b)
}

fn get_answer(lines: Vec<String>) -> usize {
    let monkeys = parse(lines);

    println!("{:?}", monkeys);
    round(&monkeys);

    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample() {
        let lines = vec![
            "Monkey 0:".to_string(),
            "    Starting items: 79, 98".to_string(),
            "    Operation: new = old * 19".to_string(),
            "    Test: divisible by 23".to_string(),
            "        If true: throw to monkey 2".to_string(),
            "        If false: throw to monkey 3".to_string(),
            "".to_string(),
            "Monkey 1:".to_string(),
            "    Starting items: 54, 65, 75, 74".to_string(),
            "    Operation: new = old + 6".to_string(),
            "    Test: divisible by 19".to_string(),
            "        If true: throw to monkey 2".to_string(),
            "        If false: throw to monkey 0".to_string(),
            "".to_string(),
            "Monkey 2:".to_string(),
            "    Starting items: 79, 60, 97".to_string(),
            "    Operation: new = old * old".to_string(),
            "    Test: divisible by 13".to_string(),
            "        If true: throw to monkey 1".to_string(),
            "        If false: throw to monkey 3".to_string(),
            "".to_string(),
            "Monkey 3:".to_string(),
            "    Starting items: 74".to_string(),
            "    Operation: new = old + 3".to_string(),
            "    Test: divisible by 17".to_string(),
            "        If true: throw to monkey 0".to_string(),
            "        If false: throw to monkey 1".to_string(),
        ];

        assert_eq!(get_answer(lines), 10605)
    }
}