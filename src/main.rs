use std::fmt;
use itertools::Itertools;
use colored::*;

#[macro_use]
extern crate log;

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
            0 => panic!("Sudoku without solutions! :/"),
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
        let rng = 1..=81;

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
    fn load(&mut self, filepath: &str) {
        let file = std::fs::read_to_string(filepath).expect("open failed");
        let chars = file.chars();
        let mut position = 0;

        for c in chars {
            match c {
                '1'..='9' => {
                    let x = position / 9 + 1;
                    let y = position % 9 + 1;

                    self.init(x, y, c.to_digit(10).unwrap() as i8);

                    position = position + 1;
                },
                '.' => {
                    position = position + 1;
                },
                _ => {}
            }
        }

        if position != 81 {
            panic!("Invalid file format");
        }
    }

    fn solve(&mut self) {
        let rng = 0..=80;

        rng.for_each(|i| {
            let x = i % 9 + 1;
            let y = i / 9 + 1;

            let cell = self.cells.get_mut(index(x, y)).expect("Unable to find cell");

            if cell.values.len() == 1 {
                let value = *cell.values.first().unwrap();
                self.set(x, y, value);
            }
        })
    }

    fn validate(&mut self, x: i8, y: i8, value: i8) {
        if x > 9 || y > 9 {
            panic!(format!("Invalid coords: ({}, {})", x, y));
        }

        if value > 9 {
            panic!(format!("Invalid value: {}", value));
        }
    }

    fn init(&mut self, x: i8, y: i8, value: i8) {
        self.validate(x, y, value);

        let cell = self.cells.get_mut(index(x, y)).expect("Unable to find cell");

        cell.values.retain(|&x| x == value);
    }

    fn set(&mut self, x: i8, y: i8, value: i8) {
        self.validate(x, y, value);

        let cell = self.cells.get_mut(index(x, y)).expect("Unable to find cell");

        cell.values.retain(|&x| x == value);

        debug!("Set cell ({}:{})={}", x, y, value);

        self.discard_row(x, y, value);
        self.discard_column(x, y, value);
        self.discard_block(x, y, value);
        self.review_blocks();
    }

    fn unset(&mut self, x: i8, y: i8, value: i8) {
        self.validate(x, y, value);

        let cell = self.cells.get_mut(index(x, y)).expect("Unable to find cell");
        let before = cell.values.len();

        cell.values.retain(|&x| x != value);

        let after = cell.values.len();

        if after == 1 && before > 1 {
            let last = *cell.values.first().unwrap();
            debug!("Ensured {} on ({},{})", last, x, y);
            self.set(x, y, last);
        }
    }

    fn discard_row(&mut self, x: i8, y: i8, value: i8) {
        let rng = 1..=9;

        rng.for_each(|i| {
            if i != y {
                debug!("Reviewing row ({}:{})", x, i);
                self.unset(x, i, value);
            }
        })
    }

    fn discard_column(&mut self, x: i8, y: i8, value: i8) {
        let rng = 1..=9;

        rng.for_each(|i| {
            if i != x {
                debug!("Reviewing column ({}:{})", i, y);
                self.unset(i, y, value);
            }
        })
    }

    fn discard_block(&mut self, x: i8, y: i8, value: i8) {
        let rng = 0..9;

        let ox = (((x - 1) / 3) * 3) + 1;
        let oy = (((y - 1) / 3) * 3) + 1;

        debug!("Reviewing block ({}:{}) with value: {} => ({}:{})", x, y, value, ox, oy);

        rng.for_each(|i| {
            let ix = ox + (i % 3);
            let iy = oy + (i / 3);

            if ix != x || iy != y {
                debug!("Unsetting block cell ({}:{}) for value {}", ix, iy, value);

                self.unset(ix, iy, value);
            }
        });
    }

    fn review_blocks(&mut self) {
        let rngx = 0..3;

        rngx.for_each(|x| {
            let x = x * 3;
            let rngy = 0..3;

            rngy.for_each(|y| {
                let y = y * 3;

                let values = 1..=9;

                values.for_each(|value| {
                    let rng = 0..9;

                    let cells: Vec<i8> = rng.filter(|i| {
                        let ix = x + (i % 3) + 1;
                        let iy = y + (i / 3) + 1;

                        let cell = self.cells.get(index(ix, iy)).expect("Unable to find cell");

                        cell.values.contains(&value)
                    }).collect();

                    if cells.len() == 1 {
                        let coord = cells.first().unwrap();
                        let ix = x + (coord % 3) + 1;
                        let iy = y + (coord / 3) + 1;

                        let cell = self.cells.get(index(ix, iy)).expect("Unable to find cell");

                        if cell.values.len() > 1 {
                            self.set(ix, iy, value);
                        }
                    }
                });
            });
        });
    }
}

fn index (x: i8, y: i8) -> usize {
    ((x - 1) * 9 + (y - 1)) as usize
}

fn main() {
    env_logger::init();
    let mut sudoku = Sudoku::default();

    sudoku.load("samples/difficult.txt");

    println!("Preview:");

    println!("{}", sudoku);

    sudoku.solve();

    println!("Solution:");

    println!("{}", sudoku);

    let cell = sudoku.cells.get_mut(index(3, 7)).expect("Unable to find cell");

    println!("(3, 7) = {:?}", cell.values);
}
