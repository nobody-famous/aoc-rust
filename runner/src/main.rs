use core::Problem;
use std::time;
use year2022::day1;
use year2022::day2;
use year2022::day3;
use year2022::day4;

fn main() {
    let problems: Vec<Problem> = vec![
        Problem::new("Day1/Part1".to_string(), day1::part1::solve),
        Problem::new("Day1/Part2".to_string(), day1::part2::solve),
        Problem::new("Day2/Part1".to_string(), day2::part1::solve),
        Problem::new("Day2/Part2".to_string(), day2::part2::solve),
        Problem::new("day3/Part1".to_string(), day3::part1::solve),
        Problem::new("day3/Part2".to_string(), day3::part2::solve),
        Problem::new("day4/Part1".to_string(), day4::part1::solve),
        Problem::new("day4/Part2".to_string(), day4::part2::solve),
    ];

    // let problems: Vec<Problem> = vec![Problem::new("day4/Part2".to_string(), day4::part2::solve)];

    let total = run_all(problems) as f64 / 1e6;

    println!("Total: {total} ms");
}

fn run_all(problems: Vec<Problem>) -> u128 {
    problems.iter().fold(0u128, |acc, prob| acc + run_one(prob))
}

fn run_one(problem: &Problem) -> u128 {
    let clock = time::Instant::now();
    let result = (problem.to_run)();
    let duration = clock.elapsed().as_nanos();

    let outcome = match result {
        Ok(_) => "OK".to_string(),
        Err(str) => str,
    };

    let label = &problem.label;
    println!("{label} [{outcome}] {} ms", duration as f64 / 1e6);

    duration
}
