use core::AocResult;

use super::utils::{parse, Grid};

pub fn solve(file_name: &str) -> AocResult<usize> {
    core::do_work(file_name, get_answer)
}

fn get_answer(lines: Vec<String>) -> AocResult<usize> {
    let mut grid = parse(lines)?;
    let initial_size = grid.count_filled();

    while drop_from(&mut grid, 500, 0) {}

    Ok(grid.count_filled() - initial_size)
}

fn drop_from(grid: &mut Box<dyn Grid>, x: isize, y: isize) -> bool {
    let Some(pt) = grid.find_next(x, y) else { return false };

    if !grid.contains(pt.x - 1, pt.y + 1) {
        drop_from(grid, pt.x - 1, pt.y + 1)
    } else if !grid.contains(pt.x + 1, pt.y + 1) {
        drop_from(grid, pt.x + 1, pt.y + 1)
    } else {
        grid.insert(pt.x, pt.y, 'O');
        true
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

        assert_eq!(get_answer(lines).unwrap(), 24)
    }
}
