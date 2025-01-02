use core::AocResult;
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

fn problem<T>(f: fn() -> AocResult<T>, expected: T) -> Box<dyn (Fn() -> String)>
where
    T: 'static + Eq + std::fmt::Debug,
{
    Box::new(move || match f() {
        Ok(actual) => {
            if actual == expected {
                String::from("OK")
            } else {
                format!("FAILED: expected {:?}, got {:?}", expected, actual)
            }
        }
        Err(e) => format!("FAILED: {:?}", e.to_string()),
    })
}

struct Year {
    label: &'static str,
    problems: Vec<Problem>,
}

fn main() {
    let years: Vec<Year> = vec![Year {
        label: "2022",
        problems: vec![
            Problem::new("Day1/Part1", problem(day1::part1::solve, 69206)),
            Problem::new("Day1/Part2", problem(day1::part2::solve, 197400)),
            Problem::new("Day2/Part1", problem(day2::part1::solve, 11150)),
            Problem::new("Day2/Part2", problem(day2::part2::solve, 8295)),
            Problem::new("Day3/Part1", problem(day3::part1::solve, 8039)),
            Problem::new("Day3/Part2", problem(day3::part2::solve, 2510)),
            Problem::new("Day4/Part1", problem(day4::part1::solve, 490)),
            Problem::new("Day4/Part2", problem(day4::part2::solve, 921)),
            Problem::new("Day5/Part1", problem(day5::part1::solve, String::from("TBVFVDZPN"))),
            Problem::new("Day5/Part2", problem(day5::part2::solve, String::from("VLCWHTDSZ"))),
            Problem::new("Day6/Part1", problem(day6::part1::solve, 1707)),
            Problem::new("Day6/Part2", problem(day6::part2::solve, 3697)),
            Problem::new("Day7/Part1", problem(day7::part1::solve, 1501149)),
            Problem::new("Day7/Part2", problem(day7::part2::solve, 10096985)),
            Problem::new("Day8/Part1", problem(day8::part1::solve, 1543)),
            Problem::new("Day8/Part2", problem(day8::part2::solve, 595080)),
            Problem::new("Day9/Part1", problem(day9::part1::solve, 6311)),
            Problem::new("Day9/Part2", problem(day9::part2::solve, 2482)),
            Problem::new("Day10/Part1", problem(day10::part1::solve, 15880)),
            Problem::new("Day10/Part2", problem(day10::part2::solve, "PLGFKAZG")),
            Problem::new("Day11/Part1", problem(day11::part1::solve, 51075)),
            Problem::new("Day11/Part2", problem(day11::part2::solve, 11741456163)),
            Problem::new("Day16/Part1", problem(day16::part1::solve, 1716)),
            Problem::new("Day16/Part2", problem(day16::part2::solve, 2504)),
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
