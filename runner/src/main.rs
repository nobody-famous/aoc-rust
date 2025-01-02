use core::AocResult;
use core::Problem;
use std::time;

fn problem<T>(label: &'static str, f: fn() -> AocResult<T>, expected: T) -> Problem
where
    T: 'static + Eq + std::fmt::Debug,
{
    Problem::new(
        label,
        Box::new(move || match f() {
            Ok(actual) => {
                if actual == expected {
                    String::from("OK")
                } else {
                    format!("FAILED: expected {:?}, got {:?}", expected, actual)
                }
            }
            Err(e) => format!("FAILED: {:?}", e.to_string()),
        }),
    )
}

struct Year {
    label: &'static str,
    problems: Vec<Problem>,
}

fn main() {
    let years: Vec<Year> = vec![Year {
        label: "2022",
        problems: vec![
            problem("Day1/Part1", year2022::day1::part1::solve, 69206),
            problem("Day1/Part2", year2022::day1::part2::solve, 197400),
            problem("Day2/Part1", year2022::day2::part1::solve, 11150),
            problem("Day2/Part2", year2022::day2::part2::solve, 8295),
            problem("Day3/Part1", year2022::day3::part1::solve, 8039),
            problem("Day3/Part2", year2022::day3::part2::solve, 2510),
            problem("Day4/Part1", year2022::day4::part1::solve, 490),
            problem("Day4/Part2", year2022::day4::part2::solve, 921),
            problem("Day5/Part1", year2022::day5::part1::solve, String::from("TBVFVDZPN")),
            problem("Day5/Part2", year2022::day5::part2::solve, String::from("VLCWHTDSZ")),
            problem("Day6/Part1", year2022::day6::part1::solve, 1707),
            problem("Day6/Part2", year2022::day6::part2::solve, 3697),
            problem("Day7/Part1", year2022::day7::part1::solve, 1501149),
            problem("Day7/Part2", year2022::day7::part2::solve, 10096985),
            problem("Day8/Part1", year2022::day8::part1::solve, 1543),
            problem("Day8/Part2", year2022::day8::part2::solve, 595080),
            problem("Day9/Part1", year2022::day9::part1::solve, 6311),
            problem("Day9/Part2", year2022::day9::part2::solve, 2482),
            problem("Day10/Part1", year2022::day10::part1::solve, 15880),
            problem("Day10/Part2", year2022::day10::part2::solve, "PLGFKAZG"),
            problem("Day11/Part1", year2022::day11::part1::solve, 51075),
            problem("Day11/Part2", year2022::day11::part2::solve, 11741456163),
            problem("Day16/Part1", year2022::day16::part1::solve, 1716),
            problem("Day16/Part2", year2022::day16::part2::solve, 2504),
        ],
    }];

    run_all(years);
}

fn run_all(years: Vec<Year>) {
    years.iter().for_each(|year| {
        println!("{}", year.label);
        println!("---------------");
        run_all_problems(&year.problems);
        println!("---------------");
    });
}

fn run_all_problems(problems: &[Problem]) {
    let total = problems.iter().fold(0u128, |acc, prob| acc + run_one(prob)) as f64 / 1e6;
    println!("Total: {total} ms");
}

fn run_one(problem: &Problem) -> u128 {
    let clock = time::Instant::now();
    let outcome = (problem.to_run)();
    let duration = clock.elapsed().as_nanos();

    let label = &problem.label;
    println!("{label} [{outcome}] {} ms", duration as f64 / 1e6);

    duration
}
