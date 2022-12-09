pub const FILE_NAME: &str = "year2022/src/day6/puzzle.txt";

pub fn get_answer(lines: Vec<String>, length: usize) -> u32 {
    let line: &String = &lines[0];
    let mut idx = 0;

    while idx + length < line.len() {
        match find_dupe(line, idx, length) {
            Some(n) => idx = n + 1,
            None => return (idx + length) as u32,
        }
    }

    0
}

fn find_dupe(line: &String, start: usize, count: usize) -> Option<usize> {
    let bytes = line.as_bytes();
    let mut flags: usize = 0;

    for idx in (start..start + count).rev() {
        let ch = bytes[idx] as char;
        let shift = ch as usize - 'a' as usize;
        let mask = 1 << shift;

        if flags & mask != 0 {
            return Some(idx);
        }

        flags |= mask;
    }

    None
}
