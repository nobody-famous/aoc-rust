use core::AocResult;

use super::utils::{get_mask, parse, walk, FILE_NAME};

const CORRECT_ANSWER: usize = 2504;

pub fn solve() -> AocResult<()> {
    core::do_work(FILE_NAME, CORRECT_ANSWER, get_answer, |a, b| a == b)
}

fn get_answer(lines: Vec<String>) -> AocResult<usize> {
    let config = parse(lines)?;
    let start = get_mask(&config.masks, &"AA".to_string())?;
    let flows = walk(&config, start, 26)?;
    let tmp = flows.iter().fold(vec![], |mut acc, entry| {
        acc.push(entry);
        acc
    });
    let mut highest = 0;

    for i in 0..tmp.len() - 1 {
        let (s1, v1) = tmp[i];

        tmp.iter().skip(i + 1).for_each(|(s2, v2)| {
            if *s1 & *s2 == 0 && *v1 + *v2 > highest {
                highest = *v1 + *v2;
            }
        })
    }

    Ok(highest)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample() {
        let lines = vec![
            String::from("Valve AA has flow rate=0; tunnels lead to valves DD, II, BB"),
            String::from("Valve BB has flow rate=13; tunnels lead to valves CC, AA"),
            String::from("Valve CC has flow rate=2; tunnels lead to valves DD, BB"),
            String::from("Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE"),
            String::from("Valve EE has flow rate=3; tunnels lead to valves FF, DD"),
            String::from("Valve FF has flow rate=0; tunnels lead to valves EE, GG"),
            String::from("Valve GG has flow rate=0; tunnels lead to valves FF, HH"),
            String::from("Valve HH has flow rate=22; tunnel leads to valve GG"),
            String::from("Valve II has flow rate=0; tunnels lead to valves AA, JJ"),
            String::from("Valve JJ has flow rate=21; tunnel leads to valve II"),
        ];

        assert_eq!(get_answer(lines).unwrap(), 1707)
    }
}
