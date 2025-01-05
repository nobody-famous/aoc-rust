use regex::Regex;

const GRID_HEIGHT: usize = 1000;
const GRID_WIDTH: usize = 1000;

pub trait Grid {
    fn contains(&self, x: isize, y: isize) -> bool;

    fn find_next(&self, x: isize, y: isize) -> Option<Point>;

    fn insert(&mut self, x: isize, y: isize, ch: char);

    fn count_filled(&self) -> usize;

    fn print(&self);
}

impl Grid for Vec<Vec<char>> {
    fn contains(&self, x: isize, y: isize) -> bool {
        y >= 0
            && y < self.len() as isize
            && x >= 0
            && x < self[0].len() as isize
            && self[y as usize][x as usize] != '.'
    }

    fn find_next(&self, x: isize, y: isize) -> Option<Point> {
        let mut pt = Point { x, y };

        while self[pt.y as usize][pt.x as usize] == '.' {
            pt.y += 1;
            if pt.y >= GRID_HEIGHT as isize {
                return None;
            }
        }

        pt.y -= 1;
        Some(pt)
    }

    fn insert(&mut self, x: isize, y: isize, ch: char) {
        self[y as usize][x as usize] = ch
    }

    fn count_filled(&self) -> usize {
        let mut count = 0;

        (0..self.len()).for_each(|y| {
            for x in 0..self[y].len() {
                if self[y][x] != '.' {
                    count += 1
                }
            }
        });

        count
    }

    fn print(&self) {
        println!();
        (0..self.len()).for_each(|y| {
            for x in 0..self[y].len() {
                print!("{}", self[y][x])
            }
            println!();
        });
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Point {
    pub x: isize,
    pub y: isize,
}

pub fn parse(lines: Vec<String>) -> Result<Box<dyn Grid>, String> {
    let points_regex_result = Regex::new(r"(\d+),(\d+)(\s.*?->\s.*?)?");
    let Ok(points_re) = points_regex_result else { return Err("Failed to create regex".into()) };

    lines
        .iter()
        .map(|line| parse_line(&points_re, line))
        .collect::<Result<Vec<Vec<Point>>, String>>()
        .and_then(populate_map)
}

fn populate_map(lines: Vec<Vec<Point>>) -> Result<Box<dyn Grid>, String> {
    Ok(lines.iter().fold(Box::new(vec![vec!['.'; GRID_HEIGHT]; GRID_WIDTH]), |mut grid, line| {
        add_points(&mut grid, line);
        grid
    }))
}

fn add_points(grid: &mut Box<dyn Grid>, line: &[Point]) {
    line.windows(2).for_each(|pair| {
        let (Some(first), Some(last)) = (pair.first(), pair.get(1)) else { return };
        add_line(grid, first, last);
    });
}

fn add_line(grid: &mut Box<dyn Grid>, first: &Point, last: &Point) {
    let mut x = first.x;
    let mut y = first.y;
    let x_diff = get_diff(first.x, last.x);
    let y_diff = get_diff(first.y, last.y);

    grid.insert(x, y, '#');

    while x != last.x || y != last.y {
        x += x_diff;
        y += y_diff;

        grid.insert(x, y, '#');
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
