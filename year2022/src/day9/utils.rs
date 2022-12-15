use std::collections::HashSet;

pub const FILE_NAME: &str = "year2022/src/day9/puzzle.txt";

#[derive(Debug)]
pub enum Direction {
    U,
    D,
    L,
    R,
}

#[derive(Debug)]
pub struct Move {
    pub dir: Direction,
    pub dist: usize,
}

impl Move {
    pub fn new(dir: Direction, dist: usize) -> Self {
        Move { dir, dist }
    }
}

pub fn parse_move(line: &String) -> Move {
    let parts: Vec<&str> = line.split(' ').collect();
    let dir = match parts[0] {
        "U" => Direction::U,
        "D" => Direction::D,
        "L" => Direction::L,
        "R" => Direction::R,
        _ => todo!(),
    };
    let dist = match parts[1].parse::<usize>() {
        Ok(n) => n,
        Err(_) => todo!(),
    };

    Move::new(dir, dist)
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct Point {
    pub x: isize,
    pub y: isize,
}

impl Point {
    pub fn new(x: isize, y: isize) -> Self {
        Point { x, y }
    }
}

#[derive(Debug)]
pub struct State {
    pub knots: Vec<Point>,
    pub seen: HashSet<Point>,
}

impl State {
    pub fn new(size: usize) -> Self {
        State {
            knots: vec![Point::new(0, 0); size],
            seen: HashSet::new(),
        }
    }
}

pub fn do_move(p1: &Point, p2: &Point, dir: &Direction) -> (Point, Point) {
    let mut dx: isize = 0;
    let mut dy: isize = 0;

    match dir {
        Direction::U => dy = -1,
        Direction::D => dy = 1,
        Direction::L => dx = -1,
        Direction::R => dx = 1,
    }

    let new_p1 = Point::new(p1.x + dx, p1.y + dy);
    let new_p2 = move_follower(&new_p1, p2);

    (new_p1, new_p2)
}

pub fn move_follower(p1: &Point, p2: &Point) -> Point {
    let diff_x = p1.x - p2.x;
    let dx = diff_x.abs();
    let diff_y = p1.y - p2.y;
    let dy = diff_y.abs();

    if dx < 2 && dy < 2 {
        Point::new(p2.x, p2.y)
    } else if dx == 0 {
        let diff = if diff_y > 0 { 1 } else { -1 };
        Point::new(p2.x, p2.y + diff)
    } else if dy == 0 {
        let diff = if diff_x > 0 { 1 } else { -1 };
        Point::new(p2.x + diff, p2.y)
    } else {
        let move_x = if diff_x > 0 { 1 } else { -1 };
        let move_y = if diff_y > 0 { 1 } else { -1 };
        Point::new(p2.x + move_x, p2.y + move_y)
    }
}
