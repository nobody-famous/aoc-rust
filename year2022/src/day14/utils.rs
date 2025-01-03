use regex::Regex;

pub struct Point {
    pub x: usize,
    pub y: usize,
}

pub fn parse(lines: Vec<String>) -> Vec<Vec<Point>> {
    let point_re = Regex::new(r"(\d+),(\d+)(\s.*?->\s.*?)?");
    let Ok(points) = point_re else { return vec![] };

    lines
        .iter()
        .map(|line| {
            points
                .captures_iter(line)
                .map(|m| match (m.get(1), m.get(2)) {
                    (None, None) | (None, Some(_)) | (Some(_), None) => Point { x: 0, y: 0 },
                    (Some(x_match), Some(y_match)) => Point {
                        x: x_match.as_str().parse::<usize>().unwrap_or(0),
                        y: y_match.as_str().parse::<usize>().unwrap_or(0),
                    },
                })
                .collect()
        })
        .collect()
}
