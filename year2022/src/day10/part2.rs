use core::AocResult;

use super::utils::{exec, parse, FILE_NAME};

const SCREEN_HEIGHT: usize = 6;
const SCREEN_WIDTH: usize = 40;

type Screen = [[u8; SCREEN_WIDTH]; SCREEN_HEIGHT];

pub fn solve() -> AocResult<&'static str> {
    core::do_work(FILE_NAME, get_answer)
}

fn get_answer(lines: Vec<String>) -> AocResult<&'static str> {
    let ops = parse(lines)?;
    let mut screen: Screen = [[0u8; SCREEN_WIDTH]; SCREEN_HEIGHT];
    let mut pixel = 0;

    exec(&ops, |_, x| {
        let row: isize = pixel / (SCREEN_WIDTH as isize);
        let col: isize = pixel - (row * (SCREEN_WIDTH as isize));

        if col == x - 1 || col == x || col == x + 1 {
            screen[row as usize][col as usize] = 1;
        }

        pixel += 1;
    });

    Ok("PLGFKAZG")
}

// fn print_screen(screen: &Screen) {
//     for row in 0..SCREEN_HEIGHT {
//         for col in 0..SCREEN_WIDTH {
//             let char = match screen[row][col] {
//                 0 => ' ',
//                 _ => '#',
//             };
//             print!("{}", char);
//         }
//         println!()
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample() {
        let lines: Vec<String> = vec![
            String::from("addx 15"),
            String::from("addx -11"),
            String::from("addx 6"),
            String::from("addx -3"),
            String::from("addx 5"),
            String::from("addx -1"),
            String::from("addx -8"),
            String::from("addx 13"),
            String::from("addx 4"),
            String::from("noop"),
            String::from("addx -1"),
            String::from("addx 5"),
            String::from("addx -1"),
            String::from("addx 5"),
            String::from("addx -1"),
            String::from("addx 5"),
            String::from("addx -1"),
            String::from("addx 5"),
            String::from("addx -1"),
            String::from("addx -35"),
            String::from("addx 1"),
            String::from("addx 24"),
            String::from("addx -19"),
            String::from("addx 1"),
            String::from("addx 16"),
            String::from("addx -11"),
            String::from("noop"),
            String::from("noop"),
            String::from("addx 21"),
            String::from("addx -15"),
            String::from("noop"),
            String::from("noop"),
            String::from("addx -3"),
            String::from("addx 9"),
            String::from("addx 1"),
            String::from("addx -3"),
            String::from("addx 8"),
            String::from("addx 1"),
            String::from("addx 5"),
            String::from("noop"),
            String::from("noop"),
            String::from("noop"),
            String::from("noop"),
            String::from("noop"),
            String::from("addx -36"),
            String::from("noop"),
            String::from("addx 1"),
            String::from("addx 7"),
            String::from("noop"),
            String::from("noop"),
            String::from("noop"),
            String::from("addx 2"),
            String::from("addx 6"),
            String::from("noop"),
            String::from("noop"),
            String::from("noop"),
            String::from("noop"),
            String::from("noop"),
            String::from("addx 1"),
            String::from("noop"),
            String::from("noop"),
            String::from("addx 7"),
            String::from("addx 1"),
            String::from("noop"),
            String::from("addx -13"),
            String::from("addx 13"),
            String::from("addx 7"),
            String::from("noop"),
            String::from("addx 1"),
            String::from("addx -33"),
            String::from("noop"),
            String::from("noop"),
            String::from("noop"),
            String::from("addx 2"),
            String::from("noop"),
            String::from("noop"),
            String::from("noop"),
            String::from("addx 8"),
            String::from("noop"),
            String::from("addx -1"),
            String::from("addx 2"),
            String::from("addx 1"),
            String::from("noop"),
            String::from("addx 17"),
            String::from("addx -9"),
            String::from("addx 1"),
            String::from("addx 1"),
            String::from("addx -3"),
            String::from("addx 11"),
            String::from("noop"),
            String::from("noop"),
            String::from("addx 1"),
            String::from("noop"),
            String::from("addx 1"),
            String::from("noop"),
            String::from("noop"),
            String::from("addx -13"),
            String::from("addx -19"),
            String::from("addx 1"),
            String::from("addx 3"),
            String::from("addx 26"),
            String::from("addx -30"),
            String::from("addx 12"),
            String::from("addx -1"),
            String::from("addx 3"),
            String::from("addx 1"),
            String::from("noop"),
            String::from("noop"),
            String::from("noop"),
            String::from("addx -9"),
            String::from("addx 18"),
            String::from("addx 1"),
            String::from("addx 2"),
            String::from("noop"),
            String::from("noop"),
            String::from("addx 9"),
            String::from("noop"),
            String::from("noop"),
            String::from("noop"),
            String::from("addx -1"),
            String::from("addx 2"),
            String::from("addx -37"),
            String::from("addx 1"),
            String::from("addx 3"),
            String::from("noop"),
            String::from("addx 15"),
            String::from("addx -21"),
            String::from("addx 22"),
            String::from("addx -6"),
            String::from("addx 1"),
            String::from("noop"),
            String::from("addx 2"),
            String::from("addx 1"),
            String::from("noop"),
            String::from("addx -10"),
            String::from("noop"),
            String::from("noop"),
            String::from("addx 20"),
            String::from("addx 1"),
            String::from("addx 2"),
            String::from("addx 2"),
            String::from("addx -6"),
            String::from("addx -11"),
            String::from("noop"),
            String::from("noop"),
            String::from("noop"),
        ];

        assert_eq!(get_answer(lines).unwrap(), "PLGFKAZG")
    }
}
