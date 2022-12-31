pub const FILE_NAME: &str = "year2022/src/day10/puzzle.txt";

#[derive(Debug)]
pub enum Op {
    Add(isize),
    Noop,
}

pub fn exec<F>(ops: &Vec<Op>, mut obs: F)
where
    F: FnMut(isize, isize) -> (),
{
    let mut cycle = 0;
    let mut x = 1;

    for op in ops {
        match op {
            Op::Add(value) => {
                cycle += 1;
                obs(cycle, x);
                cycle += 1;
                obs(cycle, x);

                x += value;
            }
            Op::Noop => {
                cycle += 1;
                obs(cycle, x);
            }
        }
    }
}

pub fn parse(lines: Vec<String>) -> Vec<Op> {
    lines
        .iter()
        .map(|line| {
            let parts: Vec<&str> = line.split(' ').collect();
            let value = if parts.len() > 1 {
                match parts[1].parse::<isize>() {
                    Ok(v) => v,
                    Err(_) => todo!(),
                }
            } else {
                0 as isize
            };

            if parts[0] == "addx" {
                Op::Add(value)
            } else {
                Op::Noop
            }
        })
        .collect()
}
