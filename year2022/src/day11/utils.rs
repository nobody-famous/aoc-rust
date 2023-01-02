use std::collections::HashMap;

use regex::Regex;

pub const FILE_NAME: &str = "year2022/src/day11/puzzle.txt";

#[derive(Debug, Clone)]
pub enum Arg {
    Value(usize),
    Old,
}

#[derive(Debug, Clone)]
pub struct Operation {
    left: Arg,
    right: Arg,
    op: char,
    true_target: usize,
    false_target: usize,
}

#[derive(Debug)]
pub struct Monkey {
    pub items: Vec<usize>,
    pub op: Operation,
    pub test: usize,
    pub inspected: usize,
}

pub fn round(monkeys: &mut HashMap<usize, Monkey>) -> Result<(), String> {
    for ndx in 0..monkeys.len() {
        if let Some(monkey) = monkeys.get_mut(&ndx) {
            let to_throw = process_monkey(monkey);

            for (ndx, worries) in to_throw {
                let target = match monkeys.get_mut(&ndx) {
                    Some(t) => t,
                    None => return Err(format!("Could not find entry for index {:?}", ndx)),
                };

                for worry in worries {
                    target.items.push(worry)
                }
            }
        } else {
            return Err(format!("Could not find entry for index {:?}", ndx));
        }
    }

    Ok(())
}

fn process_monkey(monkey: &mut Monkey) -> HashMap<usize, Vec<usize>> {
    let mut to_throw = HashMap::new();

    for item in &monkey.items {
        let new_worry = update_worry(&monkey, *item);
        let target_ndx = if new_worry % monkey.test == 0 {
            monkey.op.true_target
        } else {
            monkey.op.false_target
        };

        if !to_throw.contains_key(&target_ndx) {
            to_throw.insert(target_ndx, vec![]);
        }

        let v = match to_throw.get_mut(&target_ndx) {
            Some(v) => v,
            None => todo!(),
        };

        v.push(new_worry);
    }

    monkey.inspected += monkey.items.len();
    monkey.items.clear();

    to_throw
}

fn update_worry(monkey: &Monkey, item: usize) -> usize {
    let left = match monkey.op.left {
        Arg::Value(v) => v,
        Arg::Old => item,
    };
    let right = match monkey.op.right {
        Arg::Value(v) => v,
        Arg::Old => item,
    };

    match monkey.op.op {
        '*' => (left * right) / 3,
        '+' => (left + right) / 3,
        _ => {
            println!("Invalid operation: {:?}", monkey.op.op);
            todo!()
        }
    }
}

pub fn parse(lines: Vec<String>) -> HashMap<usize, Monkey> {
    let start_re = Regex::new(r"Monkey (\d+):");
    let items_re = Regex::new(r"\s*Starting items: (.*)");
    let op_re = Regex::new(r"\s*Operation: new = (\S+) (.) (\S+)");
    let test_re = Regex::new(r"\s*Test: divisible by (\d+)");
    let cond_re = Regex::new(r"\s*If (true|false): throw to monkey (\d+)");
    let mut monkey_map: HashMap<usize, Monkey> = HashMap::new();
    let mut cur_monkey: usize = 0;
    let mut cur_items: Vec<usize> = vec![];
    let mut cur_op: Operation = Operation {
        left: Arg::Old,
        right: Arg::Old,
        op: '+',
        true_target: 0,
        false_target: 0,
    };
    let mut cur_test: usize = 0;

    lines.iter().for_each(|line| {
        if let (Ok(start), Ok(items), Ok(op), Ok(test), Ok(cond)) =
            (&start_re, &items_re, &op_re, &test_re, &cond_re)
        {
            if start.is_match(line) {
                for cap in start.captures_iter(line) {
                    match cap[1].parse::<usize>() {
                        Ok(n) => {
                            cur_monkey = n;
                            cur_items = vec![];
                            cur_op = Operation {
                                left: Arg::Old,
                                right: Arg::Old,
                                op: '+',
                                true_target: 0,
                                false_target: 0,
                            };
                        }
                        Err(_) => todo!(),
                    }
                }
            } else if items.is_match(line) {
                for cap in items.captures_iter(line) {
                    cur_items = cap[1]
                        .split(',')
                        .map(|p| p.trim().replace(",", ""))
                        .map(|p| match p.parse::<usize>() {
                            Ok(n) => n,
                            Err(_) => todo!(),
                        })
                        .collect();
                }
            } else if op.is_match(line) {
                for cap in op.captures_iter(line) {
                    let left = parse_arg(&cap[1]);
                    let right = parse_arg(&cap[3]);
                    let op_str = &cap[2].chars().nth(0);
                    let op = match op_str {
                        Some(ch) => ch,
                        None => todo!(),
                    };

                    cur_op = Operation {
                        left,
                        op: *op,
                        right,
                        true_target: 0,
                        false_target: 0,
                    };
                }
            } else if test.is_match(line) {
                for cap in test.captures_iter(line) {
                    cur_test = match cap[1].parse::<usize>() {
                        Ok(n) => n,
                        Err(_) => todo!(),
                    };
                }
            } else if cond.is_match(line) {
                for cap in cond.captures_iter(line) {
                    let target = &cap[2].parse::<usize>();
                    if &cap[1] == "true" {
                        cur_op.true_target = match target {
                            Ok(n) => *n,
                            Err(_) => todo!(),
                        }
                    } else if &cap[1] == "false" {
                        cur_op.false_target = match target {
                            Ok(n) => *n,
                            Err(_) => todo!(),
                        }
                    } else {
                        println!("Invalid condition: {:?}", line);
                        todo!();
                    }
                }
            } else if line.len() == 0 {
                monkey_map.insert(
                    cur_monkey,
                    Monkey {
                        items: cur_items.clone(),
                        op: cur_op.clone(),
                        test: cur_test,
                        inspected: 0,
                    },
                );
            } else {
                println!("Invalid line: {:?}", line);
                todo!();
            }
        }
    });

    monkey_map.insert(
        cur_monkey,
        Monkey {
            items: cur_items.clone(),
            op: cur_op,
            test: cur_test,
            inspected: 0,
        },
    );

    monkey_map
}

fn parse_arg(data: &str) -> Arg {
    if data == "old" {
        Arg::Old
    } else {
        match data.parse::<usize>() {
            Ok(n) => Arg::Value(n),
            Err(_) => todo!(),
        }
    }
}
