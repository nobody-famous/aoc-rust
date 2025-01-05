use core::AocResult;

use super::utils::{drop_from, parse, Grid, SAND, WALL};

pub fn solve(file_name: &str) -> AocResult<usize> {
    core::do_work(file_name, get_answer)
}

fn get_answer(lines: Vec<String>) -> AocResult<usize> {
    let mut grid = parse(lines)?;

    add_floor(&mut grid);

    while !grid.contains(500, 0) {
        drop_from(&mut grid, 500, 0);
    }

    Ok(grid.count(SAND))
}

fn add_floor(grid: &mut Box<dyn Grid>) {
    let mut x = 0;
    let y = grid.highest_y() + 2;

    while grid.is_on_grid(x, y) {
        grid.insert(x, y, WALL);
        x += 1;
    }
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

        assert_eq!(get_answer(lines).unwrap(), 93)
    }
}
