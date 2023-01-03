use super::utils::{parse, round, Arg, Monkey, FILE_NAME};

const CORRECT_ANSWER: usize = 11741456163;

pub fn solve() -> Result<(), String> {
    core::do_work(FILE_NAME, CORRECT_ANSWER, get_answer, |a, b| a == b)
}

fn get_answer(lines: Vec<String>) -> Result<usize, String> {
    let mut monkeys = parse(lines)?;
    let mod_value: usize = monkeys.iter().map(|(_, m)| m.test).product();

    for _ in 0..10000 {
        if let Some(e) = round(&mut monkeys, &|monkey: &Monkey, item| {
            let left = match monkey.op.left {
                Arg::Value(v) => v,
                Arg::Old => item,
            };
            let right = match monkey.op.right {
                Arg::Value(v) => v,
                Arg::Old => item,
            };

            match monkey.op.op {
                '*' => Ok((left * right) % mod_value),
                '+' => Ok((left + right) % mod_value),
                _ => Err(format!("Invalid operation: {:?}", monkey.op.op)),
            }
        })
        .err()
        {
            return Err(e);
        }
    }

    let mut inspected: Vec<usize> = monkeys.iter().map(|(_, m)| m.inspected).collect();

    inspected.sort();
    inspected.reverse();

    Ok(inspected.iter().take(2).product())
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

        assert_eq!(get_answer(lines), Ok(2713310158))
    }
}
