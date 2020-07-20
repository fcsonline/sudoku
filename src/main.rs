use std::fmt;
use itertools::Itertools;
use colored::*;

#[derive(Debug)]
struct Cell {
    values: Vec<i8>
}

impl Default for Cell {
    fn default() -> Self {
        Cell {
            values: vec!(1, 2, 3, 4, 5, 6, 7, 8, 9)
        }
    }
}

impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.values.len() {
            0 => panic!("No cell values"),
            1 => {
                let value = self.values.first().unwrap();

                write!(f, "{}", value.to_string().bold())
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

impl Sudoku {
    fn validate(&mut self, x: i8, y: i8, value: i8) {
        if x >= 9 || y >= 9 {
            panic!(format!("Invalid coords: ({}, {})", x, y));
        }

        if value > 9 {
            panic!(format!("Invalid value: {}", value));
        }
    }

    fn set(&mut self, x: i8, y: i8, value: i8) {
        self.validate(x, y, value);

        let index = ((x - 1) * 9 + (y - 1)) as usize;

        let cell = self.cells.get_mut(index).expect("Unable to find cell");

        cell.values.retain(|&x| x == value);
    }

    fn unset(&mut self, x: i8, y: i8, value: i8) {
        self.validate(x, y, value);

        let index = ((x - 1) * 9 + (y - 1)) as usize;

        let cell = self.cells.get_mut(index).expect("Unable to find cell");

        cell.values.retain(|&x| x != value);
    }

}

fn main() {
    let mut sudoku = Sudoku::default();

    // Set values here ↓
    sudoku.set(1, 1, 9);
    sudoku.set(1, 8, 9);

    sudoku.unset(1, 2, 2);
    sudoku.unset(1, 2, 3);
    sudoku.unset(1, 2, 4);
    sudoku.unset(1, 2, 5);
    sudoku.unset(1, 2, 6);
    sudoku.unset(1, 2, 7);
    sudoku.unset(1, 2, 8);
    sudoku.unset(1, 2, 9);

    println!("{}", sudoku);
}
