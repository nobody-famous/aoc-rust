use core::Problem;
use std::time;
use year2022::day1;
use year2022::day10;
use year2022::day11;
use year2022::day16;
use year2022::day2;
use year2022::day3;
use year2022::day4;
use year2022::day5;
use year2022::day6;
use year2022::day7;
use year2022::day8;
use year2022::day9;

fn main() {
    let problems: Vec<Problem> = vec![
        Problem::new("Day1/Part1", day1::part1::solve),
        Problem::new("Day1/Part2", day1::part2::solve),
        Problem::new("Day2/Part1", day2::part1::solve),
        Problem::new("Day2/Part2", day2::part2::solve),
        Problem::new("day3/Part1", day3::part1::solve),
        Problem::new("day3/Part2", day3::part2::solve),
        Problem::new("day4/Part1", day4::part1::solve),
        Problem::new("day4/Part2", day4::part2::solve),
        Problem::new("day5/Part1", day5::part1::solve),
        Problem::new("day5/part2", day5::part2::solve),
        Problem::new("day6/part1", day6::part1::solve),
        Problem::new("day6/part2", day6::part2::solve),
        Problem::new("day7/part1", day7::part1::solve),
        Problem::new("day7/part2", day7::part2::solve),
        Problem::new("day8/part1", day8::part1::solve),
        Problem::new("day8/part2", day8::part2::solve),
        Problem::new("day9/part1", day9::part1::solve),
        Problem::new("day9/part2", day9::part2::solve),
        Problem::new("day10/part1", day10::part1::solve),
        Problem::new("day10/part2", day10::part2::solve),
        Problem::new("day11/part1", day11::part1::solve),
        Problem::new("day11/part2", day11::part2::solve),
        Problem::new("day16/part1", day16::part1::solve),
        Problem::new("day16/part2", day16::part2::solve),
    ];

    // let problems: Vec<Problem> = vec![Problem::new("day11/part2".to_string(), day11::part2::solve)];

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
        Ok(_) => String::from("OK"),
        Err(err) => err.to_string(),
    };

    let label = &problem.label;
    println!("{label} [{outcome}] {} ms", duration as f64 / 1e6);

    duration
}
