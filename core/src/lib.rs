use std::{
    fs::File,
    io::{self, BufRead},
};

pub type ProblemFn = fn() -> Result<(), String>;
pub struct Problem {
    pub label: String,
    pub to_run: ProblemFn,
}

impl Problem {
    pub fn new(label: String, to_run: ProblemFn) -> Self {
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
            _ => "".to_string(),
        })
        .collect();

    Ok(lines)
}

pub fn do_work<T>(
    file_name: &str,
    exp_answer: T,
    get_answer: impl Fn(Vec<String>) -> Result<T, String>,
    check_answer: impl Fn(&T, &T) -> bool,
) -> Result<(), String>
where
    T: std::fmt::Debug,
{
    match read_lines(file_name) {
        Ok(lines) => {
            let answer = get_answer(lines)?;

            match check_answer(&answer, &exp_answer) {
                true => Ok(()),
                false => Err(std::format!("Wrong answer {:?}", answer)),
            }
        }
        Err(e) => Err(e.to_string()),
    }
}
