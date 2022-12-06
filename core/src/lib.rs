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
