use std::collections::HashMap;

use regex::Regex;

pub const FILE_NAME: &str = "year2022/src/day11/puzzle.txt";

#[derive(Debug)]
pub struct Monkey {
    items: Vec<usize>,
    op: String,
    test: usize,
}

pub fn parse(lines: Vec<String>) -> Vec<Monkey> {
    let start_re = Regex::new(r"Monkey (\d+):");
    let items_re = Regex::new(r"\s*Starting items: (.*)");
    let op_re = Regex::new(r"\s*Operation: (.*)");
    let test_re = Regex::new(r"\s*Test: divisible by (\d+)");
    let cond_re = Regex::new(r"\s*If (true|false): throw to monkey (\d+)");
    let mut monkey_map: HashMap<usize, Monkey> = HashMap::new();
    let mut cur_monkey: usize = 0;
    let mut cur_items: Vec<usize> = vec![];
    let mut cur_op: String = String::from("");
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
                            cur_op = String::from("");
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
                    cur_op = String::from(&cap[1]);
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
                    println!("COND {:?} {:?}", &cap[1], &cap[2]);
                }
            } else if line.len() == 0 {
                monkey_map.insert(
                    cur_monkey,
                    Monkey {
                        items: cur_items.clone(),
                        op: cur_op.clone(),
                        test: cur_test,
                    },
                );
            }
        }
    });

    monkey_map.insert(
        cur_monkey,
        Monkey {
            items: cur_items.clone(),
            op: cur_op.clone(),
            test: cur_test,
        },
    );

    println!("FINISHED {:?}", monkey_map);

    vec![]
}
