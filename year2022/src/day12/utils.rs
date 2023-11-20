use std::collections::HashMap;

#[derive(Debug)]
pub struct Position {
    pub row: usize,
    pub col: usize,
}

#[derive(Debug)]
pub struct Cell {
    pub pos: Position,
    pub value: char,
}

#[derive(Debug)]
pub struct Grid {
    pub start: Position,
    pub end: Position,
    pub cells: HashMap<Position, char>,
}

pub fn parse(lines: Vec<String>) -> Vec<Cell> {
    lines_to_cells(lines)
}

fn lines_to_cells(lines: Vec<String>) -> Vec<Cell> {
    lines
        .iter()
        .enumerate()
        .flat_map(|(row, line)| line_to_cells(row, line))
        .collect()
}

fn line_to_cells(row: usize, line: &String) -> Vec<Cell> {
    line.chars()
        .enumerate()
        .map(|(col, ch)| Cell {
            pos: Position { row, col },
            value: ch,
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let lines: Vec<String> = vec![
            String::from("Sabqponm"),
            String::from("abcryxxl"),
            String::from("accszExk"),
            String::from("acctuvwj"),
            String::from("abdefghi"),
        ];

        let cells = parse(lines);
        cells.iter().for_each(|cell| println!("{:?}", cell));

        assert!(false, "Not done yet")
    }
}
