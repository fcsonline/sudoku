use std::fmt;
use itertools::Itertools;

#[derive(Debug)]
struct Cell {
    values: Vec<i8>
}

impl Default for Cell {
    fn default() -> Self {
        Cell {
            values: vec![1, 2, 3, 4, 5, 6, 7, 8, 9]
        }
    }
}

impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.values.len() {
            0 => panic!("No cell values"),
            1 => {
                let value = self.values.first().unwrap();

                write!(f, "{}", value)
            },
            _ => write!(f, "?")
        }
    }
}

#[derive(Debug)]
struct Sudoku {
    cells: Vec<Cell>
}

impl Default for Sudoku {
    fn default() -> Self {
        let rng = 0..81;

        Sudoku {
            cells: rng.map(|_| Default::default()).collect()
        }
    }
}

impl fmt::Display for Sudoku {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let data = &self
            .cells
            .iter()
            .chunks(9)
            .into_iter()
            .map ( |chunk|
                format!("{}\n", chunk.map(|c| c.to_string()).join(" "))
            )
            .collect::<String>();

        write!(f, "{}", data)
    }
}

fn main() {
    let sudoku = Sudoku::default();

    println!("{}", sudoku);
}
