pub fn do_work<T>(
    file_name: &str,
    exp_answer: T,
    get_answer: impl Fn(Vec<String>) -> T,
    check_answer: impl Fn(&T, &T) -> bool,
) -> Result<(), String>
where
    T: std::fmt::Debug,
{
    match core::read_lines(file_name) {
        Ok(lines) => {
            let answer = get_answer(lines);

            match check_answer(&answer, &exp_answer) {
                true => Ok(()),
                false => Err(std::format!("Wrong answer {:?}", answer)),
            }
        }
        Err(e) => Err(e.to_string()),
    }
}
