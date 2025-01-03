use regex::Regex;

pub struct Point {
    pub x: usize,
    pub y: usize,
}

pub fn parse(lines: Vec<String>) -> Result<Vec<Vec<Point>>, String> {
    let points_regex_result = Regex::new(r"(\d+),(\d+)(\s.*?->\s.*?)?");
    let Ok(points_re) = points_regex_result else { return Err("Failed to create regex".into()) };

    lines.iter().map(|line| parse_line(&points_re, line)).collect()
}

fn parse_line(re: &Regex, line: &str) -> Result<Vec<Point>, String> {
    re.captures_iter(line)
        .map(|m| match (m.get(1), m.get(2)) {
            (None, None) | (None, Some(_)) | (Some(_), None) => {
                Err(format!("Failed to match line: {:?}", line))
            }
            (Some(x_match), Some(y_match)) => parse_point(x_match.as_str(), y_match.as_str()),
        })
        .collect::<Result<Vec<Point>, String>>()
        .and_then(|points| {
            if points.is_empty() {
                Err(format!("No points found for line: {}", line))
            } else {
                Ok(points)
            }
        })
}

fn parse_point(x_text: &str, y_text: &str) -> Result<Point, String> {
    match (parse_usize(x_text), parse_usize(y_text)) {
        (Ok(x), Ok(y)) => Ok(Point { x, y }),
        (Ok(_), Err(e)) | (Err(e), Ok(_)) => Err(e),
        (Err(x_err), Err(y_err)) => Err(format!("Failed to parse: {:?}, {:?}", x_err, y_err)),
    }
}

fn parse_usize(text: &str) -> Result<usize, String> {
    match text.parse::<usize>() {
        Ok(n) => Ok(n),
        Err(e) => Err(format!("Failed to parse number {:?}: {:?}", text, e.to_string())),
    }
}
