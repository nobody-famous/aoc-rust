use std::{
    fs::File,
    io::{self, BufRead},
};

pub type DynError = dyn std::error::Error;
pub type AocResult<T> = Result<T, Box<DynError>>;
pub type ProblemFn = fn() -> AocResult<()>;
pub struct Problem {
    pub label: &'static str,
    pub to_run: ProblemFn,
}

impl Problem {
    pub fn new(label: &'static str, to_run: ProblemFn) -> Self {
        Problem { label, to_run }
    }
}

pub fn read_lines(file_name: &str) -> io::Result<Vec<String>> {
    let handle = File::open(file_name)?;
    let lines = io::BufReader::new(handle)
        .lines()
        .filter(|result| result.is_ok())
        .map(|result| match result {
            Ok(line) => line,
            _ => String::from(""),
        })
        .collect();

    Ok(lines)
}

pub fn do_work<T>(
    file_name: &str,
    exp_answer: T,
    get_answer: impl Fn(Vec<String>) -> AocResult<T>,
    check_answer: impl Fn(&T, &T) -> bool,
) -> AocResult<()>
where
    T: std::fmt::Debug,
{
    match read_lines(file_name) {
        Ok(lines) => {
            let answer = get_answer(lines)?;

            match check_answer(&answer, &exp_answer) {
                true => Ok(()),
                false => Err(std::format!("Wrong answer {:?}", answer).into()),
            }
        }
        Err(e) => Err(e.into()),
    }
}
