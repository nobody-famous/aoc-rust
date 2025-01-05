pub fn parse_rows(lines: Vec<String>) -> Vec<Vec<usize>> {
    lines.iter().map(|line| parse_row(line)).collect::<Vec<Vec<usize>>>()
}

fn parse_row(line: &str) -> Vec<usize> {
    line.chars().fold(vec![], |mut acc, ch| {
        acc.push(ch as usize - '0' as usize);
        acc
    })
}

pub fn create_visible(rows: &[Vec<usize>]) -> Vec<Vec<bool>> {
    let mut visible: Vec<Vec<bool>> =
        rows.iter().map(|row| row.clone().iter().map(|_| false).collect()).collect();

    set_boundary(&mut visible);
    visible
}

fn set_boundary(visible: &mut [Vec<bool>]) {
    let idx = visible.len() - 1;

    visible[0] = visible[0].iter().map(|_| true).collect();
    visible[idx] = visible[idx].iter().map(|_| true).collect();

    for idx in 1..visible.len() - 1 {
        let end = visible[idx].len() - 1;
        visible[idx][0] = true;
        visible[idx][end] = true;
    }
}
