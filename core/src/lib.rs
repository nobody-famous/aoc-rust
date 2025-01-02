use std::{
    fs::File,
    io::{self, BufRead},
};

pub type DynError = dyn std::error::Error;
pub type AocResult<T> = Result<T, Box<DynError>>;

pub struct Problem {
    pub label: &'static str,
    pub to_run: Box<dyn (Fn() -> String)>,
}

impl Problem {
    pub fn new(label: &'static str, to_run: Box<dyn (Fn() -> String)>) -> Self {
        Problem { label, to_run }
    }
}

pub fn read_lines(file_name: &str) -> io::Result<Vec<String>> {
    let handle = File::open(file_name)?;
    let lines = io::BufReader::new(handle)
        .lines()
        .filter(|result| result.is_ok())
        .map(|result| result.unwrap_or_default())
        .collect();

    Ok(lines)
}

pub fn do_work<T>(file_name: &str, get_answer: impl Fn(Vec<String>) -> AocResult<T>) -> AocResult<T>
where
    T: std::fmt::Debug,
{
    let lines = read_lines(file_name)?;
    let answer = get_answer(lines)?;

    Ok(answer)
}
