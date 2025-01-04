use core::AocResult;
use std::collections::HashSet;

use super::utils::{parse, print_grid, Point};

pub fn solve(file_name: &str) -> AocResult<usize> {
    core::do_work(file_name, get_answer)
}

fn get_answer(lines: Vec<String>) -> AocResult<usize> {
    let mut grid = parse(lines)?;
    let initial_size = grid.len();

    while drop_from(&mut grid, 500, 0) {}

    Ok(grid.len() - initial_size)
}

fn drop_from(grid: &mut HashSet<Point>, x: isize, y: isize) -> bool {
    let Some(pt) = find_next(grid, x, y) else { return false };

    if !grid.contains(&Point { x: pt.x - 1, y: pt.y + 1 }) {
        drop_from(grid, pt.x - 1, pt.y + 1)
    } else if !grid.contains(&Point { x: pt.x + 1, y: pt.y + 1 }) {
        drop_from(grid, pt.x + 1, pt.y + 1)
    } else {
        grid.insert(pt);
        true
    }
}

fn find_next(grid: &HashSet<Point>, x: isize, y: isize) -> Option<Point> {
    grid.iter()
        .filter(|pt| pt.x == x && pt.y > y)
        .map(|pt| pt.y)
        .min()
        .map(|new_y| Point { x, y: new_y - 1 })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample() {
        let lines = vec![
            String::from("498,4 -> 498,6 -> 496,6"),
            String::from("503,4 -> 502,4 -> 502,9 -> 494,9"),
        ];

        assert_eq!(get_answer(lines).unwrap(), 24)
    }
}
