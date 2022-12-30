use super::utils::{parse, walk, FILE_NAME};

const CORRECT_ANSWER: usize = 2504;

pub fn solve() -> Result<(), String> {
    core::do_work(FILE_NAME, CORRECT_ANSWER, get_answer, |a, b| a == b)
}

fn get_answer(lines: Vec<String>) -> usize {
    let config = parse(lines);
    let start = "AA".to_string();
    let flows = walk(&config, &start, 26);
    let tmp = flows.iter().fold(vec![], |mut acc, entry| {
        acc.push(entry);
        acc
    });
    let mut highest = 0;

    for i in 0..tmp.len() - 1 {
        let (s1, v1) = tmp[i];

        for j in i + 1..tmp.len() {
            let (s2, v2) = tmp[j];

            if s1.is_disjoint(s2) {
                if v1 + v2 > highest {
                    highest = v1 + v2;
                }
            }
        }
    }

    highest
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample() {
        let lines = vec![
            "Valve AA has flow rate=0; tunnels lead to valves DD, II, BB".to_string(),
            "Valve BB has flow rate=13; tunnels lead to valves CC, AA".to_string(),
            "Valve CC has flow rate=2; tunnels lead to valves DD, BB".to_string(),
            "Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE".to_string(),
            "Valve EE has flow rate=3; tunnels lead to valves FF, DD".to_string(),
            "Valve FF has flow rate=0; tunnels lead to valves EE, GG".to_string(),
            "Valve GG has flow rate=0; tunnels lead to valves FF, HH".to_string(),
            "Valve HH has flow rate=22; tunnel leads to valve GG".to_string(),
            "Valve II has flow rate=0; tunnels lead to valves AA, JJ".to_string(),
            "Valve JJ has flow rate=21; tunnel leads to valve II".to_string(),
        ];

        assert_eq!(get_answer(lines), 1707)
    }
}
