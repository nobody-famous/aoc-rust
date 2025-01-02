use core::AocResult;
use std::collections::HashMap;

use regex::Regex;

#[derive(Debug, Clone)]
pub enum Arg {
    Value(usize),
    Old,
}

#[derive(Debug, Clone)]
pub struct Operation {
    pub left: Arg,
    pub right: Arg,
    pub op: char,
    pub true_target: usize,
    pub false_target: usize,
}

#[derive(Debug)]
pub struct Monkey {
    pub items: Vec<usize>,
    pub op: Operation,
    pub test: usize,
    pub inspected: usize,
}

pub fn round(
    monkeys: &mut HashMap<usize, Monkey>,
    update_worry: &dyn for<'a> Fn(&'a Monkey, usize) -> AocResult<usize>,
) -> AocResult<()> {
    for ndx in 0..monkeys.len() {
        if let Some(monkey) = monkeys.get_mut(&ndx) {
            let to_throw = process_monkey(monkey, update_worry)?;

            for (ndx, worries) in to_throw {
                let target = match monkeys.get_mut(&ndx) {
                    Some(t) => t,
                    None => return Err(format!("Could not find entry for index {:?}", ndx).into()),
                };

                for worry in worries {
                    target.items.push(worry)
                }
            }
        } else {
            return Err(format!("Could not find entry for index {:?}", ndx).into());
        }
    }

    Ok(())
}

fn process_monkey(
    monkey: &mut Monkey,
    update_worry: &dyn for<'a> Fn(&'a Monkey, usize) -> AocResult<usize>,
) -> AocResult<HashMap<usize, Vec<usize>>> {
    let mut to_throw = HashMap::new();

    for item in &monkey.items {
        let new_worry = update_worry(monkey, *item)?;
        let target_ndx = if new_worry % monkey.test == 0 {
            monkey.op.true_target
        } else {
            monkey.op.false_target
        };

        to_throw.entry(target_ndx).or_insert_with(Vec::new);

        let v = get_to_throw_value(&mut to_throw, target_ndx)?;

        v.push(new_worry);
    }

    monkey.inspected += monkey.items.len();
    monkey.items.clear();

    Ok(to_throw)
}

fn get_to_throw_value(
    to_throw: &mut HashMap<usize, Vec<usize>>,
    ndx: usize,
) -> Result<&mut Vec<usize>, String> {
    match to_throw.get_mut(&ndx) {
        Some(v) => Ok(v),
        None => Err(format!("Could not get to throw vector {:?}", ndx)),
    }
}

pub fn parse(lines: Vec<String>) -> Result<HashMap<usize, Monkey>, String> {
    let start_re = Regex::new(r"Monkey (\d+):");
    let items_re = Regex::new(r"\s*Starting items: (.*)");
    let op_re = Regex::new(r"\s*Operation: new = (\S+) (.) (\S+)");
    let test_re = Regex::new(r"\s*Test: divisible by (\d+)");
    let cond_re = Regex::new(r"\s*If (true|false): throw to monkey (\d+)");
    let mut monkey_map: HashMap<usize, Monkey> = HashMap::new();
    let mut cur_monkey: usize = 0;
    let mut cur_items: Vec<usize> = vec![];
    let mut cur_op: Operation =
        Operation { left: Arg::Old, right: Arg::Old, op: '+', true_target: 0, false_target: 0 };
    let mut cur_test: usize = 0;

    for line in lines {
        if let (Ok(start), Ok(items), Ok(op), Ok(test), Ok(cond)) =
            (&start_re, &items_re, &op_re, &test_re, &cond_re)
        {
            if start.is_match(&line) {
                for cap in start.captures_iter(&line) {
                    let n = parse_usize(&cap[1])?;
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
            } else if items.is_match(&line) {
                for cap in items.captures_iter(&line) {
                    let pieces: Vec<String> =
                        cap[1].split(',').map(|p| p.trim().replace(",", "")).collect();

                    cur_items = vec![];
                    for piece in pieces {
                        cur_items.push(parse_usize(&piece)?);
                    }
                }
            } else if op.is_match(&line) {
                for cap in op.captures_iter(&line) {
                    let left = parse_arg(&cap[1])?;
                    let right = parse_arg(&cap[3])?;
                    let op_str = &cap[2].chars().next();
                    let op = op_str.ok_or("Could not find op string")?;

                    cur_op = Operation { left, op, right, true_target: 0, false_target: 0 };
                }
            } else if test.is_match(&line) {
                for cap in test.captures_iter(&line) {
                    cur_test = parse_usize(&cap[1])?;
                }
            } else if cond.is_match(&line) {
                for cap in cond.captures_iter(&line) {
                    let target = parse_usize(&cap[2])?;
                    if &cap[1] == "true" {
                        cur_op.true_target = target;
                    } else if &cap[1] == "false" {
                        cur_op.false_target = target;
                    } else {
                        return Err(format!("Invalid condition: {:?}", line));
                    }
                }
            } else if line.is_empty() {
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
                return Err(format!("Invalid line: {:?}", line));
            }
        }
    }

    monkey_map.insert(
        cur_monkey,
        Monkey { items: cur_items.clone(), op: cur_op, test: cur_test, inspected: 0 },
    );

    Ok(monkey_map)
}

fn parse_usize(data: &str) -> Result<usize, String> {
    match data.parse::<usize>() {
        Ok(n) => Ok(n),
        Err(e) => Err(e.to_string()),
    }
}

fn parse_arg(data: &str) -> Result<Arg, String> {
    if data == "old" {
        Ok(Arg::Old)
    } else {
        Ok(Arg::Value(parse_usize(data)?))
    }
}
