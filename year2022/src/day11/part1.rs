use core::AocResult;

use super::utils::{parse, round, Arg, Monkey, FILE_NAME};

pub fn solve() -> AocResult<usize> {
    core::do_work(FILE_NAME, get_answer)
}

fn get_answer(lines: Vec<String>) -> AocResult<usize> {
    let mut monkeys = parse(lines)?;

    for _ in 0..20 {
        round(&mut monkeys, &update_worry)?;
    }

    let mut inspected: Vec<usize> = monkeys.values().map(|m| m.inspected).collect();

    inspected.sort();
    inspected.reverse();

    Ok(inspected.iter().take(2).product())
}

fn update_worry(monkey: &Monkey, item: usize) -> AocResult<usize> {
    let left = match monkey.op.left {
        Arg::Value(v) => v,
        Arg::Old => item,
    };
    let right = match monkey.op.right {
        Arg::Value(v) => v,
        Arg::Old => item,
    };

    match monkey.op.op {
        '*' => Ok((left * right) / 3),
        '+' => Ok((left + right) / 3),
        _ => Err(format!("Invalid operation: {:?}", monkey.op.op).into()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample() {
        let lines = vec![
            String::from("Monkey 0:"),
            String::from("    Starting items: 79, 98"),
            String::from("    Operation: new = old * 19"),
            String::from("    Test: divisible by 23"),
            String::from("        If true: throw to monkey 2"),
            String::from("        If false: throw to monkey 3"),
            String::from(""),
            String::from("Monkey 1:"),
            String::from("    Starting items: 54, 65, 75, 74"),
            String::from("    Operation: new = old + 6"),
            String::from("    Test: divisible by 19"),
            String::from("        If true: throw to monkey 2"),
            String::from("        If false: throw to monkey 0"),
            String::from(""),
            String::from("Monkey 2:"),
            String::from("    Starting items: 79, 60, 97"),
            String::from("    Operation: new = old * old"),
            String::from("    Test: divisible by 13"),
            String::from("        If true: throw to monkey 1"),
            String::from("        If false: throw to monkey 3"),
            String::from(""),
            String::from("Monkey 3:"),
            String::from("    Starting items: 74"),
            String::from("    Operation: new = old + 3"),
            String::from("    Test: divisible by 17"),
            String::from("        If true: throw to monkey 0"),
            String::from("        If false: throw to monkey 1"),
        ];

        assert_eq!(get_answer(lines).unwrap(), 10605)
    }
}
