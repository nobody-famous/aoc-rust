use std::collections::HashMap;

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
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

pub fn parse(lines: Vec<String>) -> Grid {
    lines_to_cells(lines).iter().fold(
        Grid {
            start: Position { row: 0, col: 0 },
            end: Position { row: 0, col: 0 },
            cells: HashMap::new(),
        },
        |mut grid, elem| match elem.value {
            'S' => {
                grid.cells.insert(elem.pos.clone(), 'a');
                Grid {
                    start: elem.pos.clone(),
                    ..grid
                }
            }
            'E' => {
                grid.cells.insert(elem.pos.clone(), 'z');
                Grid {
                    end: elem.pos.clone(),
                    ..grid
                }
            }
            _ => {
                grid.cells.insert(elem.pos.clone(), elem.value);
                grid
            }
        },
    )
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

        let grid = parse(lines);

        assert_eq!(grid.start, Position { row: 0, col: 0 });
        assert_eq!(grid.end, Position { row: 2, col: 5 });
        assert_eq!(grid.cells.len(), 40);
    }
}
