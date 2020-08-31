use prettytable::{Table, Row, Cell};
use std::fmt;
use itertools::Itertools;
use colored::*;
use clap::{Arg, App};

#[derive(Debug)]
struct Sudoku {
    cells: Vec<Option<u8>>,
    hints: Vec<u8>
}

impl Default for Sudoku {
    fn default() -> Self {
        let rng = 1..=81;

        Sudoku {
            cells: rng.map(|_| Default::default()).collect(),
            hints: (1..=9).collect()
        }
    }
}

impl fmt::Display for Sudoku {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut table = Table::new();

        &self
            .cells
            .iter()
            .chunks(9)
            .into_iter()
            .for_each(|chunk| {
                let cells = chunk.map(|c| {
                    let text = match c {
                        Some(value) => format!("{}", value.to_string().bold()),
                        None => " ".to_string()
                    };

                    Cell::new(&text)
                }).collect();

                table.add_row(Row::new(cells));
            });

        table.printstd();
        write!(f, "")
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
                    let x = position % 9;
                    let y = position / 9;
                    let value = c.to_digit(10).unwrap() as u8;

                    self.set(x, y, Some(value));

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

    fn prepare(&mut self) {
        let mut hints: Vec<u8> = self.cells.iter().filter_map(|x| *x).collect();

        hints.sort();

        let mut data_grouped: Vec<(u8, usize)> = Vec::new();
        for (key, group) in &hints.into_iter().group_by(|elt| *elt) {
            data_grouped.push((key, group.collect::<Vec<u8>>().len()));
        }

        data_grouped.sort_by(|(_, asize), (_, bsize)| bsize.partial_cmp(asize).unwrap());

        let mut hints = data_grouped
            .iter()
            .map(|(v, _)| *v)
            .collect::<Vec<u8>>();

        for value in 1..=9 {
            if !hints.contains(&value) {
                hints.push(value);
            }
        }

        self.hints = hints;
    }

    fn solve(&mut self) {
        // FIXME: Why I need to clone?
        let hints = self.hints.clone();

        for x in 0..9 {
            for y in 0..9 {
                if self.get(x, y).is_none() {
                    for value in hints {
                        if self.possible(x, y, value) {
                            self.set(x, y, Some(value));
                            self.solve();
                            self.set(x, y, None);
                        }
                    };

                    return;
                }
            };
        };

        println!("Solved!");
        println!("{}", self);

        std::process::exit(0);
    }

    fn possible(&mut self, x: u8, y: u8, value: u8) -> bool {
        // Validate row
        let found = (0..9).find(|&x| {
            if let Some(cell) = self.get(x, y) {
                if cell == value {
                    return true;
                }
            }

            return false;
        });

        if found.is_some() {
            return false;
        }

        // Validate column
        let found = (0..9).find(|&y| {
            if let Some(cell) = self.get(x, y) {
                if cell == value {
                    return true;
                }
            }

            return false;
        });

        if found.is_some() {
            return false;
        }

        // Validate block
        let x0 = (x / 3) * 3;
        let y0 = (y / 3) * 3;

        // Validate column
        let found = (0..9).find(|&n| {
            let i = n / 3;
            let j = n % 3;

            if let Some(cell) = self.get(x0 + i, y0 + j) {
                if cell == value {
                    return true;
                }
            }

            return false;
        });

        if found.is_some() {
            return false;
        }

        return true
    }

    fn index (&self, x: u8, y: u8) -> usize {
        (x + y * 9) as usize
    }

    fn get(&mut self, x: u8, y: u8) -> Option<u8> {
        *self.cells.get(self.index(x, y)).expect(&format!("Unknown position: ({}, {})", x, y))
    }

    fn set(&mut self, x: u8, y: u8, value: Option<u8>) {
        let index = { self.index(x, y) };
        self.cells[index] = value;
    }
}

fn main() {
    let matches = App::new("Sudoku resolver")
        .version("0.1.0")
        .author("Ferran B. <fcsonline@gmail.com>")
        .about("Resolve Sudokus with a blink of an eye")
        .arg(Arg::with_name("file")
            .short("f")
            .long("file")
            .value_name("FILE")
            .help("Unresoled Sudoku")
            .takes_value(true))
        .get_matches();

    let mut sudoku = Sudoku::default();

    sudoku.load(matches.value_of("file").expect("You forgot the sudoku file"));

    println!("Preview:");
    println!("{}", sudoku);

    sudoku.prepare();
    sudoku.solve();
}
