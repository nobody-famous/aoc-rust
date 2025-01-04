use std::collections::HashSet;

use regex::Regex;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Point {
    pub x: isize,
    pub y: isize,
}

pub fn parse(lines: Vec<String>) -> Result<HashSet<Point>, String> {
    let points_regex_result = Regex::new(r"(\d+),(\d+)(\s.*?->\s.*?)?");
    let Ok(points_re) = points_regex_result else { return Err("Failed to create regex".into()) };

    lines
        .iter()
        .map(|line| parse_line(&points_re, line))
        .collect::<Result<Vec<Vec<Point>>, String>>()
        .and_then(populate_map)
}

pub fn print_grid(grid: &HashSet<Point>) {
    let (min_pt, max_pt) = get_bounds(grid);

    println!();
    for y in min_pt.y..=max_pt.y {
        for x in min_pt.x..=max_pt.x {
            print!("{}", if grid.contains(&Point { x, y }) { '#' } else { '.' });
        }
        println!();
    }
}

fn get_bounds(grid: &HashSet<Point>) -> (Point, Point) {
    let min_x = grid.iter().map(|pt| pt.x).min();
    let max_x = grid.iter().map(|pt| pt.x).max();
    let min_y = grid.iter().map(|pt| pt.y).min();
    let max_y = grid.iter().map(|pt| pt.y).max();
    let min_pt = match (min_x, min_y) {
        (None, None) | (None, Some(_)) | (Some(_), None) => todo!(),
        (Some(x), Some(y)) => Point { x, y },
    };
    let max_pt = match (max_x, max_y) {
        (None, None) | (None, Some(_)) | (Some(_), None) => todo!(),
        (Some(x), Some(y)) => Point { x, y },
    };

    (min_pt, max_pt)
}

fn populate_map(lines: Vec<Vec<Point>>) -> Result<HashSet<Point>, String> {
    Ok(lines.iter().fold(HashSet::new(), |mut set, line| {
        add_points(&mut set, line);
        set
    }))
}

fn add_points(set: &mut HashSet<Point>, line: &[Point]) {
    line.windows(2).for_each(|pair| {
        let (Some(first), Some(last)) = (pair.first(), pair.get(1)) else { return };
        add_line(set, first, last);
    });
}

fn add_line(set: &mut HashSet<Point>, first: &Point, last: &Point) {
    let mut x = first.x;
    let mut y = first.y;
    let x_diff = get_diff(first.x, last.x);
    let y_diff = get_diff(first.y, last.y);

    set.insert(Point { x, y });

    while x != last.x || y != last.y {
        x += x_diff;
        y += y_diff;

        set.insert(Point { x, y });
    }
}

fn get_diff(a: isize, b: isize) -> isize {
    match a.cmp(&b) {
        std::cmp::Ordering::Less => 1,
        std::cmp::Ordering::Equal => 0,
        std::cmp::Ordering::Greater => -1,
    }
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

fn parse_usize(text: &str) -> Result<isize, String> {
    match text.parse::<isize>() {
        Ok(n) => Ok(n),
        Err(e) => Err(format!("Failed to parse number {:?}: {:?}", text, e.to_string())),
    }
}
